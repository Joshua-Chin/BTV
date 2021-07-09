#[macro_use]
extern crate bitflags;

mod abilities;
mod challenges;
mod distributions;
mod parser;
mod rewards;
mod solver;

use std::{fs::File, io::Read, path::Path};

use clap::{AppSettings, Clap};

#[derive(Clap)]
#[clap(version="1.0", author="Joshua Chin")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Args {
    /// The path of the input file.
    input: String,
}

fn main() {
    let args = Args::parse();

    let input = {
        let path = Path::new(&args.input);
        let mut file = File::open(path).unwrap();
        let mut input= String::new();
        file.read_to_string(&mut input).unwrap();
        input
    };

    println!("Parsing challenges");
    let challenges = parser::parse(input).unwrap();
    
    let solutions = solver::solve(&challenges);
    
    for solution in solutions {
        println!("Cost: {}", solution.cost);
        println!("Success chance: {}", solution.log_proba.exp());
        println!("Order: {:?}", solution.order);
        println!("Abilities: {:?}", solution.abilities);
        println!();
    }
}
