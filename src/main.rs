use std::ops::Deref;

#[macro_use]
extern crate bitflags;

mod abilities;
mod challenges;
mod distributions;
mod non_nan;
mod parser;
mod rewards;
mod solver;

fn main() {
    let solver = challenges::Solver::new(vec![(5, 4), (2, 3), (1, 6)]);
    let h1 = solver.convex_hull(rewards::Rewards::NONE, 0);
    let h2 = solver.convex_hull(rewards::Rewards::NONE, 1);
    let h3 = solver.convex_hull(rewards::Rewards::NONE, 2);
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
    println!("{:?}", solver::merge_hulls(&hulls).len());
}
