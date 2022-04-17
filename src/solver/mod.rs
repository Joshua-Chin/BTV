use itertools::Itertools;

use crate::{abilities::Abilities, challenges::Challenge, rewards::Rewards};

use self::{challenge::ChallengeSolver, merge::{MAX_TOTAL_COST, merge_hulls}};

mod challenge;
mod merge;

/// The number of cost values to considers. The total cost will be even.
const TOTAL_COSTS: usize = (MAX_TOTAL_COST as usize) / 2 + 1;

#[derive(Debug)]
pub struct SolutionVertex {
    pub cost: u32,
    pub log_proba: f32,
    pub order: Vec<usize>,
    pub abilities: Vec<Abilities>,
}

pub fn solve(challenges: &Vec<Challenge>) -> Vec<SolutionVertex> {
    // Precompute challenge setups
    println!("Generating individual challenge setups");
    let challenge_solutions = ChallengeSolver::new(challenges);
    // Separate challenges with and without rewards
    let mut no_rewards = Vec::new();
    let mut rewards = Vec::new();
    for (idx, challenge) in challenges.iter().enumerate() {
        match challenge.reward {
            Rewards::NONE => no_rewards.push(idx),
            _ => rewards.push(idx),
        }
    }
    // Generate solution table
    const INIT: Option<SolutionVertex> = None;
    let mut table = [INIT; TOTAL_COSTS];
    // Loop over each permutation of challenges.
    println!("Searching for optimal challenge order");
    let mut hulls = Vec::new();
    hulls.reserve_exact(challenges.len());
    for head in rewards.iter().permutations(rewards.len()) {
        let mut rewards = Rewards::NONE;
        hulls.clear();
        // Sequence the challenges with no rewards after the other challenges.
        let permutation: Vec<usize> = head.into_iter().chain(no_rewards.iter()).cloned().collect();
        // Collect the hulls.
        for idx in &permutation {
            hulls.push(challenge_solutions.solution_to(rewards, *idx));
            rewards |= challenges[*idx].reward;
        }
        // Merge the hulls.
        let merged_hull = merge_hulls(&hulls);
        // Add hull to the table.
        for candidate in merged_hull {
            let idx = candidate.cost as usize / 2;
            if let Some(vertex) = &table[idx] {
                if vertex.log_proba > candidate.log_proba {
                    continue;
                }
            }
            table[idx] = Some(SolutionVertex {
                cost: candidate.cost,
                log_proba: candidate.log_proba,
                order: permutation.clone(),
                abilities: candidate.configuration,
            })
        }
    }
    // Gather the vertices from the table
    let mut output: Vec<SolutionVertex> = Vec::new();
    for entry in table {
        match entry {
            Some(vertex) => {
                if output.last().map_or(true, |v| vertex.log_proba > v.log_proba) {
                    output.push(vertex);
                }
            },
            None => {},
        }
    }
    output
}
