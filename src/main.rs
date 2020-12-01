use std::ops::Deref;

use challenges::Challenge;
use rewards::Rewards;

#[macro_use]
extern crate bitflags;

mod abilities;
mod challenges;
mod convex_hulls;
mod distributions;
mod ord_sub;
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
    let solver = convex_hulls::Solver::new(&challenges);
    let h1 = solver.convex_hull(rewards::Rewards::NONE, 0);
    let h2 = solver.convex_hull(rewards::Rewards::NONE, 0);
    let h3 = solver.convex_hull(rewards::Rewards::NONE, 0);
    let hulls = [
        h1.deref(),
        h2.deref(),
        h3.deref(),
        h1.deref(),
        h2.deref(),
        h3.deref(),
        h1.deref(),
        h2.deref(),
        h3.deref(),
        h1.deref(),
        h2.deref(),
    ];
    println!("{:?}", solver::merge_hulls(&hulls));
}
