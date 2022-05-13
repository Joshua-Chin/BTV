#[macro_use]
extern crate bitflags;

mod abilities;
mod challenges;
mod distributions;
mod parser;
mod rewards;
mod solver;

use std::{fs::File, io::Read, ops::Add, path::Path};

use clap::{AppSettings, Clap};

use crate::abilities::Abilities;

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
        let proba = solution.log_proba.exp();
        if proba < 0.1 {
            continue;
        }
        println!("Cost: {}", solution.cost);
        println!("Success chance: {}", proba);
        println!("Order: {:?}", solution.order);
        println!("Totals: {:?}", solution.abilities.iter().fold(Abilities::new(), Abilities::add));
        println!("Abilities:");
        for abilities in solution.abilities {
            println!("{:?}", abilities);
        }
        println!();
    }
}
