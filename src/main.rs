use challenges::Challenge;
use rewards::Rewards;

#[macro_use]
extern crate bitflags;

mod abilities;
mod challenges;
mod distributions;
mod parser;
mod rewards;
mod solver;

fn main() {
    let challenges = vec![Challenge {
        name: "challenge".to_string(),
        abilities: 4,
        target_idx: 5,
        reward: Rewards::NONE,
    }];
    let solver = solver::convex_hulls::ConvexHulls::new(&challenges);
    let h1 = solver.get(rewards::Rewards::NONE, 0);
    println!("{:?}", h1);
}
