use std::fmt::Debug;

use rand::Rng;

use crate::graph::{To, UndirectedGraph};

#[derive(Debug)]
pub struct Transition<T> {
    to: T,
    step_cost: i32,
}

pub trait SearchProblem<'a, T> {
    fn initial_state(&self) -> &'a T;
    fn graph(&self) -> &'a dyn UndirectedGraph<T>;
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

impl<'a, T: PartialEq> SearchProblem<'a, T> for GenericSearchProblem<'a, T> {
    fn initial_state(&self) -> &'a T {
        self.initial
    }

    fn graph(&self) -> &'a dyn UndirectedGraph<T> {
        self.graph
    }

    fn is_goal(&self, state: &T) -> bool {
        self.goal == state
    }
}

#[derive(Debug)]
pub struct SearchData<'a, T> {
    path: Vec<&'a T>,
    cost: i64,
}

pub trait SearchSolver<'a, T> {
    fn solve(&self, problem: &dyn SearchProblem<'a, T>) -> SearchData<'a, T>;
}

pub struct RandomSearchSolver;

impl RandomSearchSolver {
    pub(crate) fn new() -> RandomSearchSolver {
        RandomSearchSolver {}
    }
}

impl<'a, T: Debug + Clone> SearchSolver<'a, T> for RandomSearchSolver {
    fn solve(&self, problem: &dyn SearchProblem<'a, T>) -> SearchData<'a, T> {
        let mut state = SearchData {
            path: vec![],
            cost: 0,
        };
        let mut on = problem.initial_state();
        state.path.push(on); // initial state
        let mut random = rand::thread_rng();
        loop {
            let actions = problem.graph().get_connections(&on);
            let len = actions.len();
            assert_ne!(len, 0, "we can't take any actions");
            let index = random.gen_range(0, len);
            let To(to, step_cost) = actions[index];
            state.cost += step_cost;
            state.path.push(to);
            if problem.is_goal(to) {
                break;
            }
            on = to;
        }
        state
    }
}
