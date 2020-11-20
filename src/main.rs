#[macro_use]
extern crate bitflags;

mod abilities;
mod challenges;
mod distributions;
mod rewards;

fn main() {
    let mut solver = challenges::Solver::new(vec!((5, 4), (2, 3), (1, 6)));
    println!("{:?}", solver.convex_hull(rewards::Rewards::NONE, 0));
}
