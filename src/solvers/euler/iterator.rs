use crate::solvers::differetials::DifferentialEquation;
use crate::solvers::euler::Euler;
use crate::solvers::euler::state::State;

pub struct EulerIterator<E> {
    solver: Euler<E>,
    current: State,
}

impl<E> EulerIterator<E> {
    pub fn new(solver: Euler<E>, current: State) -> Self {
        Self { solver, current }
    }
}

impl<E: DifferentialEquation> Iterator for EulerIterator<E> {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        let next_y = self.solver.next(self.current.t, self.current.y);
        let next_t = self.current.t + self.solver.step;

        self.current = State {
            t: next_t,
            y: next_y,
        };

        Some(self.current.clone())
    }
}
