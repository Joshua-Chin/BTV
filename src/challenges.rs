use crate::rewards::Rewards;

/// The maximum number of abilities that a challenge can use.
pub const MAX_ABILITIES: usize = 16;

/// The set of possible challenge targets, in increasing order.
pub const TARGET_SET: [usize; 9] = [10, 11, 20, 25, 30, 35, 40, 45, 70];


/// A BTV challenge.
#[derive(Debug, Eq, PartialEq)]
pub struct Challenge {
    /// Name of the challenge.
    pub name: String,

    /// The index of the target within `TARGET_SET`.
    pub target_idx: usize,

    /// The maximum number of abilities that can used on this challenge.
    pub abilities: u32,

    /// The rewards offered by completing this challenge.
    pub reward: Rewards,
}
