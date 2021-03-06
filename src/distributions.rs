use crate::abilities::Ability;
use crate::rewards::Rewards;

/// The number of distribution values to track. The target specified is at most 70.
const TARGETS: usize = 71;

/// The distribution the sum of multiple ability rolls.
#[derive(Clone, Debug)]
pub struct Distribution {
    // The complementary cumulative distribution. `ccdf[target]` is the probability that
    // the outcome of the rolls is less than or equal to `target`.
    ccdf: [f32; TARGETS],
}

impl Distribution {
    /// Returns a new distribution with no ability rolls.
    pub fn new() -> Distribution {
        let mut ccdf = [0.0; TARGETS];
        ccdf[0] = 1.0;
        Distribution { ccdf }
    }

    /// Returns the probability that outcome is at least `target`.
    pub fn at_least(&self, target: usize) -> f32 {
        self.ccdf[target]
    }

    /// Returns a distribution equal to the sum of this distribution and the given ability roll.
    pub fn add_ability(&self, ability: Ability, rewards: Rewards) -> Distribution {
        match ability {
            Ability::Atmosphere => self.add_die(
                ability.cost() + (rewards.contains(Rewards::ATMOSPHERE_RANGE) as u32),
                false,
            ),
            Ability::Diction => self.add_die(
                ability.cost() + (rewards.contains(Rewards::DICTION_RANGE) as u32),
                rewards.contains(Rewards::DICTION_STRENGTH),
            ),
            Ability::Precision => self.add_die(
                ability.cost() + (rewards.contains(Rewards::PRECISION_RANGE) as u32),
                rewards.contains(Rewards::PRECISION_STRENGTH),
            ),
            Ability::Calmness => {
                self.add_die(ability.cost(), rewards.contains(Rewards::CALMNESS_STRENGTH))
            }
            Ability::Style => {
                if rewards.contains(Rewards::STYLE_EXPLODING) {
                    self.add_exploding_style()
                } else {
                    self.add_die(ability.cost(), false)
                }
            }
            _ => self.add_die(ability.cost(), false),
        }
    }

    fn add_die(&self, range: u32, strength: bool) -> Distribution {
        let mut output = Distribution::new();
        let mut window = range as f32;
        let range_us = range as usize;

        if strength {
            for i in 1..TARGETS {
                output.ccdf[i] = window / (range as f32);
                window += self.ccdf[i - 1];
                window += if i + 1 >= range_us {
                    self.ccdf[i + 1 - range_us]
                } else {
                    1.0
                };
                window -= 2.0
                    * if i >= range_us {
                        self.ccdf[i - range_us]
                    } else {
                        1.0
                    };
            }
        } else {
            for i in 1..TARGETS {
                output.ccdf[i] = window / (range as f32);
                window += self.ccdf[i];
                window -= if i >= range_us {
                    self.ccdf[i - range_us]
                } else {
                    1.0
                };
            }
        }

        output
    }

    fn add_exploding_style(&self) -> Distribution {
        // Compute the exploding roll
        let mut explosion = Distribution::new();
        for i in 1..TARGETS {
            explosion.ccdf[i] = if i >= 18 { self.ccdf[i - 18] } else { 1.0 };
        }
        explosion = explosion.add_die(2, false).add_die(20, false);

        // Compute the base roll
        let mut output = self.add_die(18, false);

        // Merge the base and exploding rolls
        for i in 1..TARGETS {
            output.ccdf[i] = 0.9 * output.ccdf[i] + 0.1 * explosion.ccdf[i];
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let dist = Distribution::new();
        assert_eq!(dist.at_least(0), 1.0);
        assert_eq!(dist.at_least(1), 0.0);
    }

    #[test]
    fn test_add_die() {
        let dist = Distribution::new().add_die(8, false);
        assert_eq!(dist.at_least(1), 1.0);
        assert_eq!(dist.at_least(2), 0.875);
        assert_eq!(dist.at_least(8), 0.125);
        assert_eq!(dist.at_least(9), 0.0);
    }

    #[test]
    fn test_add_strength_die() {
        let dist = Distribution::new().add_die(8, true);
        assert_eq!(dist.at_least(2), 1.0);
        assert_eq!(dist.at_least(3), 0.875);
        assert_eq!(dist.at_least(8), 0.25);
        assert_eq!(dist.at_least(9), 0.0);
    }

    #[test]
    fn test_add_exploding_style() {
        let dist = Distribution::new().add_exploding_style();
        assert_eq!(dist.at_least(1), 1.0);
        assert_eq!(dist.at_least(19), 0.10);
        assert!((dist.at_least(40) - 0.0025).abs() < 1e6);
        assert_eq!(dist.at_least(41), 0.0);
    }

    #[test]
    fn test_add_precision() {
        let mut dist = Distribution::new();
        for _ in 0..11 {
            dist = dist.add_ability(Ability::Precision, Rewards::NONE);
        }
        println!("{}", dist.at_least(45));
    }
}
