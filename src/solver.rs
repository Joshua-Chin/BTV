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
                heap.push((OrdSub::new(point.slope).unwrap(), point.cost, 1));
            }
            None => {}
        }
    }

    let mut output = vec![];
    output.push((total_cost, total_log_probability, configuration));

    while let Some((_slope, hull, idx)) = heap.pop() {
        let h = hulls[hull as usize].as_ref();
        total_cost += h[idx].cost - h[idx - 1].cost;
        total_log_probability += h[idx].log_proba - h[idx - 1].log_proba;
        configuration[hull as usize] = hulls[hull as usize].as_ref()[idx].abilities;
        if let Some(point) = h.get(idx + 1) {
            heap.push((OrdSub::new(point.slope).unwrap(), hull, idx + 1));
        }
        output.push((total_cost, total_log_probability, configuration));
    }

    output
}
