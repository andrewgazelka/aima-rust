use crate::graph::{To, UndirectedGraph};
use rand::Rng;
use serde::export::Formatter;
use std::fmt;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Transition<T> {
    to: T,
    step_cost: i32,
}

pub trait SearchProblem<T> {
    fn initial_state(&self) -> &T;
    fn graph(&self) -> &dyn UndirectedGraph<T>;
    fn is_goal(&self, state: &T) -> bool;
}

pub struct GenericSearchProblem<'a, T: PartialEq> {
    initial: &'a T,
    graph: &'a dyn UndirectedGraph<T>,
    goal: &'a T,
}

impl<'a, T: PartialEq> GenericSearchProblem<'a, T> {
    pub fn new(
        initial: &'a T,
        goal: &'a T,
        graph: &'a dyn UndirectedGraph<T>,
    ) -> GenericSearchProblem<'a, T> {
        GenericSearchProblem {
            initial,
            graph,
            goal,
        }
    }
}

impl<'a, T: PartialEq> SearchProblem<T> for GenericSearchProblem<'a, T> {
    fn initial_state(&self) -> &T {
        self.initial
    }

    fn graph(&self) -> &dyn UndirectedGraph<T> {
        self.graph
    }

    fn is_goal(&self, state: &T) -> bool {
        self.goal == state
    }
}

pub struct RandomSearchSolver<'a, T: Debug + Clone> {
    path: Vec<&'a T>,
    cost: i64,
    problem: &'a dyn SearchProblem<T>,
}

impl<'a, T: Debug + Clone> Debug for RandomSearchSolver<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("RandomSearchSolver")
            .field("path", &self.path)
            .field("cost", &self.cost)
            .finish()
    }
}

impl<T: Debug + Clone> RandomSearchSolver<'_, T> {
    pub fn new(problem: &dyn SearchProblem<T>) -> RandomSearchSolver<T> {
        RandomSearchSolver {
            path: vec![],
            cost: 0,
            problem,
        }
    }
    pub fn solve(&mut self) {
        let search_problem = self.problem;
        let mut state = search_problem.initial_state();
        self.path.push(state); // initial state
        let mut random = rand::thread_rng();
        loop {
            let actions = search_problem.graph().get_connections(&state);
            let len = actions.len();
            assert_ne!(len, 0, "we can't take any actions");
            let index = random.gen_range(0, len);
            let To(to, step_cost) = actions[index];
            self.cost += step_cost;
            self.path.push(to);
            if search_problem.is_goal(to) {
                break;
            }
            state = to;
        }
        println!("solved {:?}", self);
    }
}
