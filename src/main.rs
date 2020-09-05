use crate::agents::SimpleReflexAgent;
use crate::environments::{Environment, RandomEnvironment};

mod environments;
mod agents;

fn main() {
    let agent = SimpleReflexAgent::new();
    let mut env = RandomEnvironment::new(Box::new(agent));
    env.run(1_000);
    println!("env {:?}", env);
}

/*
Agent type
    Simple reflex - current precept -> action
 */
