// use crate::agents::SimpleReflexAgent;
// use crate::environments::{Environment, RandomEnvironment};

use crate::search::{RomaniaSearchProblem, SearchSolver};

mod environments;
mod agents;
mod search;

fn main() {
    let problem = RomaniaSearchProblem::init();
    let mut solver = SearchSolver::new();
    solver.solve(&problem);
    println!("done!");
}

/*
Agent type
    Simple reflex - current precept -> action
 */
