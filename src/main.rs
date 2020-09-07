// use crate::agents::SimpleReflexAgent;
// use crate::environments::{Environment, RandomEnvironment};

use std::error::Error;

use crate::graph::{StringNode, UndirectedGraphImpl};
use crate::search::{GenericSearchProblem, RandomSearchSolver, SearchData, SearchSolver};

mod graph;
mod search;

fn main() -> Result<(), Box<dyn Error>> {
    let graph = UndirectedGraphImpl::romania()?;
    let a = StringNode::from("Arad".to_string());
    let b = StringNode::from("Bucharest".to_string());
    let problem = GenericSearchProblem::new(&a, &b, &graph);
    let solver = RandomSearchSolver::new();
    let data: SearchData<StringNode> = solver.solve(&problem);
    println!("solved {:?}", data);
    Ok(())
}

/*
Agent type
    Simple reflex - current precept -> action
 */
