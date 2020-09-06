// use crate::agents::SimpleReflexAgent;
// use crate::environments::{Environment, RandomEnvironment};

use std::fs;

use iced::{Application, Settings};
use toml::Value;
use toml::value::Table;

use crate::graph::UndirectedGraphImpl;
use crate::search::{RandomSearchSolver, RomaniaSearchProblem, SearchDisplay};

mod environments;
mod agents;
mod search;
mod graph;


fn main() {
    println!("yeehaw");
    let graph = UndirectedGraphImpl::new("data/romania_locations.toml", "data/romania_distances.toml");
    println!("graph {:?}", graph);
}

/*
Agent type
    Simple reflex - current precept -> action
 */
