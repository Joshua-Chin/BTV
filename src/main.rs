#[macro_use]
extern crate bitflags;

mod abilities;
mod challenges;
mod distributions;
mod parser;
mod rewards;
mod solver;

fn main() {
    let challenges = parser::parse(include_str!("test_input.txt")).unwrap();
    let solver = solver::convex_hulls::ConvexHulls::new(&challenges);
    let h1 = solver.get(rewards::Rewards::NONE, 0);
    println!("{:?}", h1);
}
