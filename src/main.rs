mod dimacs;
mod solver;

use rustc_hash::FxHashMap;
use std::fs::File;

use dimacs::Parser;
use solver::{Solver, SolverState};

fn main() {
    let file = File::open("./sorted.cnf").unwrap();
    let input = Parser::new(file);
    let mut influence: FxHashMap<isize, usize> = FxHashMap::default();
    input.into_iter().enumerate().for_each(|(idx, clause)| {
        println!("{idx}: {:?}", clause);
        clause
            .into_iter()
            .for_each(|literal| *influence.entry(literal).or_default() += 1)
    });

    println!("{:?}", influence);

    let mut solver = Solver::new(influence);

    let file = File::open("./sorted.cnf").unwrap();
    let input = Parser::new(file);
    let result = input
        .into_iter()
        .map(|clause| solver.add_clause(clause))
        .all(|s| s == SolverState::Running);

    println!("{result}");

    // let formula = Formula::from_2d(input);
    // formula.solve();
}
