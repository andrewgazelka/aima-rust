use crate::agents::Action::*;
use crate::environments::{State};
use std::fmt::Debug;

pub enum Action {
    Move(usize),
    Clean,
}


pub trait Agent: Debug {
    fn act(&mut self, environment_state: &State) -> Action;
}

#[derive(Debug)]
pub struct SimpleReflexAgent;

impl SimpleReflexAgent {
    pub fn new() -> SimpleReflexAgent {
        SimpleReflexAgent {}
    }
}

impl Agent for SimpleReflexAgent {
    fn act(&mut self, environment_state: &State) -> Action {
        let State { dirty, location } = *environment_state;
        if dirty {
            return Clean;
        }
        return match location {
            0 => Move(1),
            _ => Move(0)
        };
    }
}
