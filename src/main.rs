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
    solver::solve(&challenges);
}
