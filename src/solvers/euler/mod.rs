pub mod iterator;
pub mod state;
use super::euler::iterator::EulerIterator;
use super::euler::state::State;
use crate::solvers::differetials::DifferentialEquation;

pub struct Euler<E> {
    equation: E,
    step: f64,
}

impl<E: DifferentialEquation + Clone> Euler<E> {
    pub fn new(equation: &E, step: f64) -> Self {
        Self {
            equation: equation.clone(),
            step: step,
        }
    }
}

impl<E: DifferentialEquation> Euler<E> {
    pub fn next(&self, t: f64, y: f64) -> f64 {
        y + self.step * self.equation.derivative(t, y)
    }

    pub fn iterate(self, t0: f64, y0: f64) -> EulerIterator<E> {
        EulerIterator::new(self, State { t: t0, y: y0 })
    }
}
