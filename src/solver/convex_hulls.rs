use std::collections::HashMap;

use crate::{
    abilities::{Ability, Abilities},
    challenges::Challenge,
    challenges::MAX_ABILITIES,
    challenges::TARGET_SET,
    distributions::Distribution,
    rewards::Rewards,
};

const MAX_COST: usize = 300;
const COSTS: usize = MAX_COST / 2 + 1;

#[derive(Debug)]
pub struct HullPoint {
    pub cost: u32,
    pub log_proba: f32,
    pub slope: f32,
    pub abilities: Abilities,
}

pub type ConvexHull = Vec<HullPoint>;

#[derive(Copy, Clone, Default)]
struct Solution {
    proba: f32,
    abilities: Abilities,
}

type Solutions = [[[Solution; TARGET_SET.len()]; MAX_ABILITIES + 1]; COSTS];

pub struct ConvexHulls {
    cache: HashMap<Rewards, Vec<ConvexHull>>,
}

impl ConvexHulls {
    pub fn new(challenges: &Vec<Challenge>) -> ConvexHulls {
        let mut cache = HashMap::new();
        for rewards in Rewards::combinations() {
            convex_hulls(rewards, challenges, &mut cache);
        }
        ConvexHulls { cache }
    }

    pub fn get(&self, rewards: Rewards, idx: usize) -> &ConvexHull {
        &self.cache[&rewards][idx]
    }
}

fn convex_hulls(
    rewards: Rewards,
    challenges: &Vec<Challenge>,
    output: &mut HashMap<Rewards, Vec<ConvexHull>>,
) {
    if output.contains_key(&rewards) {
        return;
    }

    // Compute the optimal ability set for each target given a fixed cost and abilities
    let mut solutions = [[[Default::default(); TARGET_SET.len()]; MAX_ABILITIES + 1]; COSTS];
    search(
        rewards & !Rewards::ADDITIONAL_ABILITY,
        0,
        0,
        0,
        Abilities::new(),
        &Distribution::new(),
        &mut solutions,
    );

    // Sweep over abilities used
    for cost in 0..COSTS {
        for target in 0..TARGET_SET.len() {
            let mut best = solutions[cost][0][target];
            for abilities in 1..MAX_ABILITIES + 1 {
                let value = &mut solutions[cost][abilities][target];
                if best.proba > value.proba {
                    *value = best;
                } else {
                    best = *value;
                }
            }
        }
    }

    // Compute convex hulls
    for key in [rewards, rewards ^ Rewards::ADDITIONAL_ABILITY].iter() {
        let mut cache_value = Vec::with_capacity(challenges.len());
        let mut probabilities = [Default::default(); COSTS];

        for challenge in challenges.iter() {
            for cost in 0..probabilities.len() {
                let target = challenge.target_idx as usize;
                let abilities = challenge.abilities as usize
                    + (key.contains(Rewards::ADDITIONAL_ABILITY) as usize);
                probabilities[cost] = solutions[cost][abilities][target];
            }
            cache_value.push(convex_hull(&probabilities));
        }

        output.insert(*key, cache_value);
    }
}

fn search(
    rewards: Rewards,
    idx: usize,
    cost: usize,
    total_abilities: usize,
    mut abilities: Abilities,
    distribution: &Distribution,
    solutions: &mut Solutions,
) {
    if cost > MAX_COST {
        return;
    }

    for (idx, target) in TARGET_SET.iter().enumerate() {
        let entry = &mut solutions[cost / 2][total_abilities][idx];
        if distribution.at_least(*target) >= entry.proba {
            *entry = Solution {
                proba: distribution.at_least(*target),
                abilities,
            };
        }
    }

    if idx >= Ability::values().len() || total_abilities >= MAX_ABILITIES {
        return;
    }

    let ability = Ability::values()[idx];
    let mut new_distribution = distribution.clone();

    for rolls in 0..(MAX_ABILITIES - total_abilities + 1) {
        search(
            rewards,
            idx + 1,
            cost + rolls * (ability.cost() as usize),
            total_abilities + rolls,
            abilities,
            &new_distribution,
            solutions,
        );
        new_distribution = new_distribution.add_ability(ability, rewards);
        abilities[ability] += 1;
    }
}

fn convex_hull<T: AsRef<[Solution]>>(curve: T) -> ConvexHull {
    let mut hull: ConvexHull = vec![];

    for (idx, solution) in curve.as_ref().iter().enumerate() {
        // Ignore values within epsilon of 0
        if solution.proba <= 1e-6 {
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

        hull.push(HullPoint {cost: cost as u32, log_proba, slope, abilities: solution.abilities});
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

    fn convex_hull(challenges: &Vec<Challenge>, rewards: Rewards, idx: usize) -> ConvexHull {
        let mut output = HashMap::new();
        convex_hulls(rewards, challenges, &mut output);
        output.remove(&rewards).unwrap().swap_remove(idx)
    }

    #[test]
    fn test_convex_hull_no_rewards() {
        let challenges = vec![Challenge {
            name: "challenge".to_string(),
            abilities: 4,
            target_idx: 5,
            reward: Rewards::NONE,
        }];
        let solution = convex_hull(&challenges, Rewards::NONE, 0);
        assert_eq!(solution.len(), 26);
        assert_eq!(solution[0].cost, 36);
        assert!((solution[0].log_proba + 4.0943) < 1e-5);
    }

    #[test]
    fn test_convex_hull_exploding_style() {
        let challenges = vec![Challenge {
            name: "challenge".to_string(),
            abilities: 4,
            target_idx: 5,
            reward: Rewards::NONE,
        }];
        let solution = convex_hull(&challenges, Rewards::STYLE_EXPLODING, 0);
        assert_eq!(solution[0].cost, 20);
        assert!((solution[0].log_proba + -3.5935) < 1e-5);
    }
}
