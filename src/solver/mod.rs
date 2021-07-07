pub mod challenge_solver;

/* 

use std::collections::BinaryHeap;

use crate::{abilities::Abilities, challenges::Challenge, ord_sub::OrdSub};

const CHALLENGES: usize = 2;
const MAX_COST: usize = 1000;
const COSTS: usize = MAX_COST / 2 + 1;

pub struct Solution {
    cost: u32,
    log_proba: f32,
    configuration: [Abilities; CHALLENGES],
}

pub fn solve(challenges: &[Challenge; CHALLENGES]) -> Vec<Solution> {
    vec![]
}

pub fn merge_hulls<T: AsRef<[HullPoint]> + Sized>(
    hulls: &[T; CHALLENGES],
) -> Vec<(u32, f32, [Abilities; CHALLENGES])> {
    // Initialize running variables
    let mut total_cost = 0;
    let mut total_log_probability = 0.0;
    let mut configuration = [Abilities::new(); CHALLENGES];

    // Initialize heap
    let mut heap = BinaryHeap::new();
    heap.reserve_exact(hulls.len());

    // Iterate through each hull
    for (idx, hull) in hulls.iter().enumerate() {
        match hull.as_ref().get(0) {
            Some(point) => {
                total_cost += point.cost;
                total_log_probability += point.log_proba;
                configuration[idx] = point.abilities;
            }
            None => {
                return vec![];
            }
        }
        match hull.as_ref().get(1) {
            Some(point) => {
                heap.push((OrdSub::new(point.slope).unwrap(), point.cost, idx, 1));
            }
            None => {}
        }
    }

    let mut output = vec![];
    output.push((total_cost, total_log_probability, configuration));

    while let Some((_slope, cost, hull, idx)) = heap.pop() {
        let h = hulls[hull].as_ref();
        total_cost += h[idx].cost - h[idx - 1].cost;
        total_log_probability += h[idx].log_proba - h[idx - 1].log_proba;
        configuration[hull as usize] = hulls[hull as usize].as_ref()[idx].abilities;
        if let Some(point) = h.get(idx + 1) {
            heap.push((OrdSub::new(point.slope).unwrap(), point.cost - cost, hull, idx + 1));
        }
        output.push((total_cost, total_log_probability, configuration));
    }

    output
}
*/