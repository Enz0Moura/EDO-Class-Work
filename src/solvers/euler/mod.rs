pub trait DifferentialEquation {
    fn derivative(&self, t: f64, y: f64) -> f64;
}

pub struct Euler<E> {
    equation: E,
    step: f64,
}

impl<E: DifferentialEquation> Euler<E> {
    pub fn new(equation: E, step: f64) -> Self {
        Self { equation, step }
    }

    pub fn step(&self, t: f64, y: f64) -> f64 {
        y + self.step * self.equation.derivative(t, y)
    }
}

