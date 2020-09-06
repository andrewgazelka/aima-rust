use rand::{random, Rng};

use crate::agents::Action::*;
use crate::agents::Agent;

#[derive(Debug)]
struct Square {
    dirty: bool
}

impl Square {
    fn random() -> Square {
        return Square {
            dirty: random(),
        };
    }
}

pub trait Environment {
    fn run(&mut self, interval: i32);
    fn get_state(&self) -> State;
}


pub struct State {
    pub dirty: bool,
    pub location: usize,
}

///
#[derive(Debug)]
pub struct RandomEnvironment {
    squares: Vec<Square>,
    clean_square_count: i32,
    robot_location: usize,
    agent: Box<dyn Agent>,
}

impl RandomEnvironment {
    pub fn new(agent: Box<dyn Agent>) -> RandomEnvironment {
        return RandomEnvironment {
            squares: (0..2).map(|_| Square::random()).collect(),
            robot_location: rand::thread_rng().gen_range(0, 2),
            agent,
            clean_square_count: 0,
        };
    }
}

impl Environment for RandomEnvironment {
    fn run(&mut self, interval: i32) {
        for _ in 0..interval {
            let state = self.get_state();
            let agent = self.agent.as_mut();
            match agent.act(&state) {
                Move(x) => self.robot_location = x,
                Clean => self.squares[self.robot_location].dirty = false
            }
            let clean_count = self.squares.iter().fold(0, |x, s| if s.dirty { x } else { x + 1 });
            self.clean_square_count += clean_count;
        }
    }

    fn get_state(&self) -> State {
        return State {
            dirty: self.squares[self.robot_location].dirty,
            location: self.robot_location,
        };
    }
}
