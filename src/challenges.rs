use std::collections::HashMap;

const COUNT : usize = 11;

pub struct Solver {
    challenges: [(u32, u32); COUNT],
    cache: HashMap<u32, [Vec<(u32, u32)>; COUNT]>,
}

impl Solver {
    pub fn new(challenges: [(u32, u32); COUNT]) -> Solver {
        Solver{ challenges, cache: HashMap::new() }
    }

    pub fn convex_hull(&self, idx: usize) -> &Vec<(u32, u32)> {
        &self.cache.get(&0).unwrap()[3]
    }
}

struct Solution {
    
}