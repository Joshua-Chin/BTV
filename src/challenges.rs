use std::{cell::Ref, cell::RefCell, ops::Deref};

use crate::{
    abilities::{Ability, AbilitySet},
    distributions::Distribution,
    rewards::Rewards,
};

const MAX_COST: usize = 300;
const COSTS: usize = MAX_COST / 2 + 1;
const MAX_ABILITIES: usize = 16;
const TARGETS: [usize; 9] = [10, 11, 20, 25, 30, 35, 40, 45, 70];

pub type HullPoint = (u32, f32, f32, AbilitySet);
pub type ConvexHull = Vec<HullPoint>;
type Solutions = [[[(f32, AbilitySet); TARGETS.len()]; MAX_ABILITIES + 1]; COSTS];

pub struct Solver {
    challenges: Vec<(u32, u32)>,
    cache: Vec<RefCell<Option<Vec<ConvexHull>>>>,
}

impl Solver {
    pub fn new(challenges: Vec<(u32, u32)>) -> Solver {
        let mut cache = vec![];
        cache.reserve_exact(256);
        for _ in 0..256 {
            cache.push(Default::default());
        }
        Solver { challenges, cache }
    }

    pub fn convex_hull(
        &self,
        rewards: Rewards,
        idx: usize,
    ) -> impl Deref<Target = ConvexHull> + '_ {
        if self.cache[rewards.bits() as usize].borrow().is_none() {
            // Compute the optimal ability set for each target given a fixed cost and abilities
            let mut solutions =
                [[[(0.0, AbilitySet::new()); TARGETS.len()]; MAX_ABILITIES + 1]; COSTS];
            search(
                rewards & !Rewards::ADDITIONAL_ABILITY,
                0,
                0,
                0,
                AbilitySet::new(),
                &Distribution::new(),
                &mut solutions,
            );

            // Sweep over abilities used
            for cost in 0..COSTS {
                for target in 0..TARGETS.len() {
                    let mut best = solutions[cost][0][target];
                    for abilities in 1..MAX_ABILITIES + 1 {
                        let value = &mut solutions[cost][abilities][target];
                        if best.0 > value.0 {
                            *value = best;
                        } else {
                            best = *value;
                        }
                    }
                }
            }

            // Compute convex hulls
            for key in [rewards, rewards ^ Rewards::ADDITIONAL_ABILITY].iter() {
                let mut cache_value = vec![];
                cache_value.reserve_exact(self.challenges.len());

                let mut probabilities = [(0.0, AbilitySet::new()); COSTS];

                for challenge in self.challenges.iter() {
                    for cost in 0..probabilities.len() {
                        let target = challenge.0 as usize;
                        let abilities = challenge.1 as usize
                            + (key.contains(Rewards::ADDITIONAL_ABILITY) as usize);
                        probabilities[cost] = solutions[cost][abilities][target];
                    }
                    cache_value.push(convex_hull(&probabilities));
                }
                *self.cache[rewards.bits() as usize].borrow_mut() = Some(cache_value);
            }
        }
        Ref::map(self.cache[rewards.bits() as usize].borrow(), |x| {
            x.as_ref().unwrap().get(idx).unwrap()
        })
    }
}

fn search(
    rewards: Rewards,
    idx: usize,
    cost: usize,
    total_abilities: usize,
    mut abilities: AbilitySet,
    distribution: &Distribution,
    solutions: &mut Solutions,
) {
    if cost > MAX_COST {
        return;
    }

    for (idx, target) in TARGETS.iter().enumerate() {
        let entry = &mut solutions[cost / 2][total_abilities][idx];
        if distribution.at_least(*target) >= entry.0 {
            *entry = (distribution.at_least(*target), abilities);
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

fn convex_hull<T: AsRef<[(f32, AbilitySet)]>>(curve: T) -> ConvexHull {
    let mut hull: ConvexHull = vec![];

    for (idx, (proba, abilities)) in curve.as_ref().iter().enumerate() {
        // Ignore values within epsilon of 0
        if *proba <= 1e-6 {
            continue;
        }

        let log_proba = proba.ln();
        let cost = 2 * idx;

        let mut slope = f32::MAX;
        while let Some(prev) = hull.last() {
            slope = (prev.1 - log_proba) / ((prev.0 as f32) - (cost as f32));
            if slope < prev.2 {
                break;
            }
            hull.pop();
        }

        hull.push((cost as u32, log_proba, slope, *abilities));
    }

    while let Some(point) = hull.last() {
        if point.2 > 0.0 {
            break;
        }
        hull.pop();
    }

    hull
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convex_hull_no_rewards() {
        let solver = Solver::new(vec![(5, 4)]);
        let solution = solver.convex_hull(Rewards::NONE, 0);
        assert_eq!(solution.len(), 26);
        assert_eq!(solution[0].0, 36);
        assert!((solution[0].1 + 4.0943) < 1e-5);
    }

    #[test]
    fn test_convex_hull_exploding_style() {
        let solver = Solver::new(vec![(5, 4)]);
        let solution = solver.convex_hull(Rewards::STYLE_EXPLODING, 0);
        assert_eq!(solution[0].0, 20);
        assert!((solution[0].1 + -3.5935) < 1e-5);
    }
}
