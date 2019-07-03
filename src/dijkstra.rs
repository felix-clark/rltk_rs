use std::f32::MAX;
use super::BaseMap;
use std::mem;
use rayon::prelude::*;

#[allow(dead_code)]
pub struct DijkstraMap {
    pub map : Vec<f32>,
    size_x : i32,
    size_y : i32,
    max_depth : f32
}

#[allow(dead_code)]
impl DijkstraMap {
    pub fn new(size_x : i32, size_y: i32, starts: &Vec<i32>, map: &BaseMap, max_depth : f32) -> DijkstraMap {
        let mut result : Vec<f32> = Vec::with_capacity((size_x * size_y) as usize);
        for _i in 0 .. (size_x * size_y) { result.push(MAX) }
        let mut d = DijkstraMap{ map : result, size_x : size_x, size_y : size_y, max_depth : max_depth};
        DijkstraMap::build(&mut d, starts, map);
        return d;
    }

    pub fn new_empty(size_x : i32, size_y: i32, max_depth : f32) -> DijkstraMap {
        let mut result : Vec<f32> = Vec::with_capacity((size_x * size_y) as usize);
        for _i in 0 .. (size_x * size_y) { result.push(MAX) }
        let d = DijkstraMap{ map : result, size_x : size_x, size_y : size_y, max_depth : max_depth};
        return d;
    }

    #[inline(always)]
    fn add_if_open(dm : &DijkstraMap, idx : i32, open_list : &mut Vec<(i32, f32)>, closed_list : &mut Vec<bool>, new_depth : f32) {
        if new_depth > dm.max_depth { return; }
        if closed_list[idx as usize] { return; }

        closed_list[idx as usize] = true;
        open_list.push((idx, new_depth));
    }

    pub fn clear(dm: &mut DijkstraMap) {
        //dm.map.iter_mut().map(|x| *x = MAX).count();
        dm.map.par_iter_mut().for_each(|x| *x = MAX);
    }

    pub fn build(dm : &mut DijkstraMap, starts: &Vec<i32>, map: &BaseMap) {
        let mapsize : usize = (dm.size_x * dm.size_y) as usize;
        let mut open_list : Vec<(i32, f32)> = Vec::with_capacity(mapsize*2);
        let mut closed_list : Vec<bool> = Vec::with_capacity(mapsize);
        for _i in 0..mapsize { closed_list.push(false); }

        for start in starts.iter() {
            // Clearing vec in debug mode is stupidly slow, so we do it the hard way!
            unsafe { open_list.set_len(0); }
            // Zeroing the buffer is far too slow, so we're doing it the C way
            unsafe {
                libc::memset(
                    closed_list.as_mut_ptr() as _,
                    0,
                    closed_list.len() * mem::size_of::<bool>(),
                );
            }
            open_list.push((*start, 0.0));

            while !open_list.is_empty() {
                let last_idx = open_list.len()-1;
                let current_tile = open_list[last_idx];
                let tile_idx = current_tile.0;
                let depth = current_tile.1;
                unsafe { open_list.set_len(last_idx); }

                if dm.map[tile_idx as usize] > depth {
                    dm.map[tile_idx as usize] = depth;

                    let exits = map.get_available_exits(tile_idx);
                    for exit in exits.iter() {
                        DijkstraMap::add_if_open(dm, exit.0, &mut open_list, &mut closed_list, depth + exit.1);
                    }
                }
            }
        }
    }

    pub fn find_lowest_exit(dm : &DijkstraMap, position : i32, map: &BaseMap) -> Option<i32> {
        let mut exits = map.get_available_exits(position);

        for exit in exits.iter_mut() {
            exit.1 = dm.map[exit.0 as usize] as f32;
        }

        if exits.is_empty() { return None; }
        exits.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap() );

        return Some(exits[0].0);
    }

    pub fn find_highest_exit(dm : &DijkstraMap, position : i32, map: &BaseMap) -> Option<i32> {
        let mut exits = map.get_available_exits(position);

        for exit in exits.iter_mut() {
            exit.1 = dm.map[exit.0 as usize] as f32;
        }

        if exits.is_empty() { return None; }
        exits.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap() );

        return Some(exits[0].0);
    }
}
