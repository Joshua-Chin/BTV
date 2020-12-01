use crate::rewards::Rewards;

pub const MAX_ABILITIES: usize = 16;
pub const TARGETS: [usize; 9] = [10, 11, 20, 25, 30, 35, 40, 45, 70];

#[derive(Debug, Eq, PartialEq)]
pub struct Challenge {
    pub name: String,
    pub target_idx: usize,
    pub abilities: u32,
    pub reward: Rewards,
}
