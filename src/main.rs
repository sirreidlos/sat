mod dimacs;
mod solver;

use std::fs::File;

use dimacs::Parser;
use solver::{Solver, SolverState};

fn main() {
    let file = File::open("./tutorial.cnf").unwrap();
    let input = Parser::new(file);

    let mut solver = Solver::new(input.get_variables());

    let result = input
        .into_iter()
        .map(|clause| solver.add_clause(clause))
        .all(|s| s == SolverState::Running);

    println!("{result}");

    // let formula = Formula::from_2d(input);
    // formula.solve();
}
