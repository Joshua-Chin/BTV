use std::collections::BinaryHeap;

use crate::ord_sub::OrdSub;
use crate::{abilities::AbilitySet, convex_hulls::HullPoint};

const CHALLENGES: usize = 11;

pub fn merge_hulls<T: AsRef<[HullPoint]> + Sized>(
    hulls: &[T; CHALLENGES],
) -> Vec<(u32, f32, [AbilitySet; CHALLENGES])> {
    // Initialize running variables
    let mut total_cost = 0;
    let mut total_log_probability = 0.0;
    let mut configuration = [AbilitySet::new(); CHALLENGES];

    // Initialize heap
    let mut heap = BinaryHeap::new();
    heap.reserve_exact(hulls.len());

    // Iterate through each hull
    for (idx, hull) in hulls.iter().enumerate() {
        match hull.as_ref().get(0) {
            Some((cost, log_probability, _slope, abilities)) => {
                total_cost += cost;
                total_log_probability += log_probability;
                configuration[idx] = *abilities;
            }
            None => {
                return vec![];
            }
        }
        match hull.as_ref().get(1) {
            Some((_, _, slope, _abilities)) => {
                heap.push((OrdSub::new(*slope).unwrap(), idx, 1));
            }
            None => {}
        }
    }

    let mut output = vec![];
    output.push((total_cost, total_log_probability, configuration));

    while let Some((_slope, hull, idx)) = heap.pop() {
        let h = hulls[hull].as_ref();
        total_cost += h[idx].0 - h[idx - 1].0;
        total_log_probability += h[idx].1 - h[idx - 1].1;
        configuration[hull] = hulls[hull].as_ref()[idx].3;
        if let Some((_, _, slope, _abilities)) = h.get(idx + 1) {
            heap.push((OrdSub::new(*slope).unwrap(), hull, idx + 1));
        }
        output.push((total_cost, total_log_probability, configuration));
    }

    output
}
