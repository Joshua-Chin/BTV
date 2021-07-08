use std::collections::HashMap;

use crate::{
    abilities::{Abilities, Ability},
    challenges::{Challenge, MAX_ABILITIES, TARGET_SET},
    distributions::Distribution,
    rewards::Rewards,
};

const EPSILON: f32 = 1e-6;

/// Calculates and caches the optimal ability configurations of a set of challenges.
pub struct ChallengeSolver {
    cache: HashMap<Rewards, Vec<ConvexHull>>,
}

impl ChallengeSolver {
    /// Returns a challenge solver given a set of challenges to consider.
    pub fn new(challenges: &Vec<Challenge>) -> ChallengeSolver {
        let mut cache = HashMap::new();
        for rewards in Rewards::combinations() {
            solve(rewards, challenges, &mut cache);
        }
        ChallengeSolver { cache: HashMap::new() }
    }

    /// Returns the convex hull over the optimal ability configurations.
    ///
    /// # Arguments
    /// - *rewards*: The rewards available while attempting the challenge.
    /// - *challenge_idx*: The index of the challenge.
    pub fn solution_to(&self, rewards: Rewards, challenge_idx: usize) -> &ConvexHull {
        &self.cache[&rewards][challenge_idx]
    }
}

/// Calculates the convex hull over optimal ability configurations given a set of rewards.
///
/// The convex hull is written to the output map. Both the given reward setup with and without the
/// additional ability rewards are computed simultaneously.
fn solve(
    rewards: Rewards,
    challenges: &Vec<Challenge>,
    output: &mut HashMap<Rewards, Vec<ConvexHull>>,
) {
    // Bail early if this rewards has already been calculated.
    if output.contains_key(&rewards) {
        return;
    }

    // Compute the optimal ability setup for each target given a fixed cost and abilities.
    let mut solutions = empty_table();
    search(
        // Ignore the additional ability reward. It will be handled in the convex hull calculations.
        rewards & !Rewards::ADDITIONAL_ABILITY,
        0,
        0,
        0,
        Abilities::new(),
        &Distribution::new(),
        &mut solutions,
    );

    // Ensure that each entry contains the optimal setup given a fixed cost and max abilities.
    ability_cummax(&mut solutions);

    // Handle both with and with the addition ability reward simultaneously.
    for key in [rewards, rewards ^ Rewards::ADDITIONAL_ABILITY].iter() {
        let mut cache_value = Vec::with_capacity(challenges.len());
        let mut probabilities = [Default::default(); COSTS];

        for challenge in challenges.iter() {
            // Gather the entries corresponding to the challenge.
            for cost in 0..probabilities.len() {
                let target = challenge.target_idx as usize;
                let abilities = challenge.abilities as usize
                    + (key.contains(Rewards::ADDITIONAL_ABILITY) as usize);
                probabilities[cost] = solutions[cost][abilities][target];
            }
            // Compute the convex hull over the entries.
            cache_value.push(convex_hull(&probabilities));
        }

        output.insert(*key, cache_value);
    }
}

/// The maximum cost to consider of a single single challenge.
const MAX_COST: usize = 300;

/// The number of different cost values to consider. The cost of any ability combination must be even.
const COSTS: usize = MAX_COST / 2 + 1;

/// An entry in the search table.
#[derive(Copy, Clone, Debug, Default)]
struct SearchEntry {
    /// The success probabilty of the ability configuration.
    proba: f32,

    /// The ability configuration.
    abilities: Abilities,
}

/// A mapping from (cost, abilties_used, target) triples to ability configurations.
type SearchTable = [[[SearchEntry; TARGET_SET.len()]; MAX_ABILITIES + 1]; COSTS];

/// Returns an empty search table.
fn empty_table() -> SearchTable {
    [[[Default::default(); TARGET_SET.len()]; MAX_ABILITIES + 1]; COSTS]
}

/// Search for optimal ability setups of (cost, abilities used, target) triples.
///
/// Each entry will match the cost and abilities used exactly. Results are written to `solutions`.
fn search(
    rewards: Rewards,
    ability_idx: usize,
    cost: usize,
    total_abilities: usize,
    mut abilities: Abilities,
    distribution: &Distribution,
    table: &mut SearchTable,
) {
    if cost > MAX_COST {
        return;
    }

    for (idx, target) in TARGET_SET.iter().enumerate() {
        let entry = &mut table[cost / 2][total_abilities][idx];
        if distribution.at_least(*target) >= entry.proba {
            *entry = SearchEntry {
                proba: distribution.at_least(*target),
                abilities,
            };
        }
    }

    if ability_idx >= Ability::values().len() || total_abilities >= MAX_ABILITIES {
        return;
    }

    let ability = Ability::values()[ability_idx];
    let mut new_distribution = distribution.clone();

    for rolls in 0..(MAX_ABILITIES - total_abilities + 1) {
        search(
            rewards,
            ability_idx + 1,
            cost + rolls * (ability.cost() as usize),
            total_abilities + rolls,
            abilities,
            &new_distribution,
            table,
        );
        new_distribution = new_distribution.add_ability(ability, rewards);
        abilities[ability] += 1;
    }
}

/// Computes the cummulative maximum over the ability axis.
fn ability_cummax(table: &mut SearchTable) {
    // Sweep over abilities used
    for cost in 0..COSTS {
        for target in 0..TARGET_SET.len() {
            let mut best = table[cost][0][target];
            for abilities in 1..MAX_ABILITIES + 1 {
                let value = &mut table[cost][abilities][target];
                if best.proba > value.proba {
                    *value = best;
                } else {
                    best = *value;
                }
            }
        }
    }
}

/// Ability configuration of a challenge on the convex hull.
#[derive(Debug)]
pub struct Vertex {
    /// The cost of the ability configuration.
    pub cost: u32,

    /// The log probability of passing the challenge.
    pub log_proba: f32,

    /// The (log probability / cost) slope with respect to the previous vertex on the convex hull.
    pub slope: f32,

    /// The ability configuration.
    pub abilities: Abilities,
}

/// A convex hull over challenge solutions.
pub type ConvexHull = Vec<Vertex>;

/// Returns the convex hull over an array of challenge solutions.
fn convex_hull<T: AsRef<[SearchEntry]>>(curve: T) -> ConvexHull {
    let mut hull: ConvexHull = vec![];

    for (idx, solution) in curve.as_ref().iter().enumerate() {
        // Ignore values within epsilon of 0
        if solution.proba <= EPSILON {
            continue;
        }

        let cost = 2 * idx;
        let log_proba = solution.proba.ln();

        let mut slope = f32::MAX;
        while let Some(prev) = hull.last() {
            slope = (prev.log_proba - log_proba) / ((prev.cost as f32) - (cost as f32));
            if slope < prev.slope {
                break;
            }
            hull.pop();
        }

        hull.push(Vertex {
            cost: cost as u32,
            log_proba,
            slope,
            abilities: solution.abilities,
        });
    }

    while let Some(point) = hull.last() {
        if point.slope > 0.0 {
            break;
        }
        hull.pop();
    }

    hull
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solve(challenges: &Vec<Challenge>, rewards: Rewards, idx: usize) -> ConvexHull {
        let mut output = HashMap::new();
        super::solve(rewards, challenges, &mut output);
        output.remove(&rewards).unwrap().swap_remove(idx)
    }

    #[test]
    fn test_solve_no_rewards() {
        let challenges = vec![Challenge {
            name: "challenge".to_string(),
            abilities: 4,
            target_idx: 5,
            reward: Rewards::NONE,
        }];
        let solution = solve(&challenges, Rewards::NONE, 0);
        assert_eq!(solution.len(), 26);
        assert_eq!(solution[0].cost, 36);
        assert!((solution[0].log_proba + 4.0943) < EPSILON);
    }

    #[test]
    fn test_solve_exploding_style() {
        let challenges = vec![Challenge {
            name: "challenge".to_string(),
            abilities: 4,
            target_idx: 5,
            reward: Rewards::NONE,
        }];
        let solution = solve(&challenges, Rewards::STYLE_EXPLODING, 0);
        assert_eq!(solution[0].cost, 20);
        assert!((solution[0].log_proba + -3.5935) < EPSILON);
    }
}
