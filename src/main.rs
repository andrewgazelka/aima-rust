// use crate::agents::SimpleReflexAgent;
// use crate::environments::{Environment, RandomEnvironment};

use crate::graph::UndirectedGraphImpl;
use std::error::Error;

mod agents;
mod environments;
mod graph;
mod search;

fn main() -> Result<(), Box<dyn Error>> {
    let graph = UndirectedGraphImpl::romania()?;
    println!("graph {:?}", graph);
    Ok(())
}

/*
Agent type
    Simple reflex - current precept -> action
 */
