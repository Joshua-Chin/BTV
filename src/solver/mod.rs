use itertools::Itertools;

use crate::{challenges::Challenge, rewards::Rewards};

use self::{challenge::ChallengeSolver, merge::{MAX_TOTAL_COST, merge_hulls}};

mod challenge;
mod merge;

/// The number of cost values to considers. The total cost will be even.
const TOTAL_COSTS: usize = (MAX_TOTAL_COST as usize) / 2 + 1;

pub fn solve(challenges: &Vec<Challenge>) {
    let challenge_solutions = ChallengeSolver::new(challenges);

    let mut no_rewards = Vec::new();
    let mut rewards = Vec::new();
    for (idx, challenge) in challenges.iter().enumerate() {
        match challenge.reward {
            Rewards::NONE => no_rewards.push(idx),
            _ => rewards.push(idx),
        }
    }

    for permutation in rewards.iter().permutations(rewards.len()) {
        let mut rewards = Rewards::NONE;
        let mut hulls = Vec::new();
        hulls.reserve_exact(challenges.len());
        // Sequence the challenges with no rewards after the other challenges.
        // Collect the hulls.
        for idx in permutation {
            hulls.push(challenge_solutions.solution_to(rewards, *idx));
            rewards |= challenges[*idx].reward;
        }
        // Merge the hulls.
        let merged_hull = merge_hulls(&hulls);

        // TODO: Add hull to the table.

        // TODO: Compute convex hull over table.
    }
}
