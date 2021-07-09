use std::{cmp::Ordering, collections::BinaryHeap};

use crate::abilities::Abilities;

use super::challenge::ChallengeConvexHull;

/// A vertex on the convex hull of the challenges solution.
#[derive(Debug)]
pub struct ChallengesVertex {
    /// The total cost of the challenges.
    pub cost: u32,

    /// The log probability of succeeding in all challenges.
    pub log_proba: f32,

    /// The configuration of abilities for each challenge.
    pub configuration: Vec<Abilities>,
}

type ChallengesConvexHull = Vec<ChallengesVertex>;

/// The maximum cost to consider spending on the challenges.
pub const MAX_TOTAL_COST: u32 = 1000;

pub fn merge_hulls<T: AsRef<ChallengeConvexHull> + Sized>(hulls: &Vec<T>) -> ChallengesConvexHull {
    // Initialize running variables
    let mut cost = 0;
    let mut log_proba = 0.0;
    let mut configuration = {
        let mut configuration = Vec::new();
        configuration.reserve_exact(hulls.len());
        for _ in 0..hulls.len() {
            configuration.push(Abilities::new());
        }
        configuration
    };

    // Initialize heap
    let mut heap = BinaryHeap::new();
    heap.reserve_exact(hulls.len());

    // Iterate through each hull
    for (idx, hull) in hulls.iter().enumerate() {
        match hull.as_ref().get(0) {
            Some(point) => {
                cost += point.cost;
                log_proba += point.log_proba;
                configuration[idx] = point.abilities;
            }
            None => {
                return vec![];
            }
        }
        match hull.as_ref().get(1) {
            Some(point) => {
                let prev_cost = hull.as_ref()[0].cost;
                heap.push((OrdF32(point.slope), point.cost - prev_cost, idx, 1));
            }
            None => {}
        }
    }

    let mut output = vec![];
    output.push(ChallengesVertex {
        cost,
        log_proba,
        configuration: configuration.clone(),
    });

    while let Some((_slope, marginal_cost, hull, idx)) = heap.pop() {
        // Update tracking variables.
        let h = hulls[hull].as_ref();
        cost += marginal_cost;
        log_proba += h[idx].log_proba - h[idx - 1].log_proba;
        configuration[hull as usize] = hulls[hull as usize].as_ref()[idx].abilities;
        // Check for early exit.
        if cost > MAX_TOTAL_COST {
            break;
        }
        // Push the next point of the hull onto the heap
        if let Some(point) = h.get(idx + 1) {
            heap.push((OrdF32(point.slope), point.cost - h[idx].cost, hull, idx + 1));
        }
        // Add the vertex to the output.
        output.push(ChallengesVertex {
            cost,
            log_proba,
            configuration: configuration.clone(),
        });
    }

    output
}

#[derive(PartialEq, PartialOrd)]
struct OrdF32(f32);

impl Eq for OrdF32 {}

impl Ord for OrdF32 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::solver::challenge::ChallengeVertex;

    use super::*;

    #[test]
    fn test_merge_hulls() {
        let hulls = vec![
            vec![
                ChallengeVertex{cost: 5, log_proba: -1.0, slope: 0.0, abilities: Abilities::new()},
                ChallengeVertex{cost: 6, log_proba: -0.5, slope: 0.5, abilities: Abilities::new()},
            ],
            vec![
                ChallengeVertex{cost: 1, log_proba: -2.0, slope: 0.0, abilities: Abilities::new()},
                ChallengeVertex{cost: 5, log_proba: -1.0, slope: 0.25, abilities: Abilities::new()},
            ],
        ];
        let merged_hull = merge_hulls(&hulls);
        assert_eq!(merged_hull.len(), 3);
        assert_eq!(merged_hull[0].cost, 6);
        assert_eq!(merged_hull[0].log_proba, -3.0);
        assert_eq!(merged_hull[1].cost, 7);
        assert_eq!(merged_hull[1].log_proba, -2.5);
        assert_eq!(merged_hull[2].cost, 11);
        assert_eq!(merged_hull[2].log_proba, -1.5);
    }
}