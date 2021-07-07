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
    let solver = solver::challenge_solver::ChallengeSolver::new(&challenges);
    let h1 = solver.solution_to(rewards::Rewards::NONE, 0);
    println!("{:?}", h1);
}
