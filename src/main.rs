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
    let solutions = solver::solve(&challenges);
    for solution in solutions {
        println!("Solution");
        println!("Cost: {}", solution.cost);
        println!("Success chance: {}", solution.log_proba.exp());
        println!("Order: {:?}", solution.order);
        println!("Abilities: {:?}", solution.abilities);
        println!();
    }
}
