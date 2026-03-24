use crate::solvers::differetials::DifferentialEquation;

#[derive(Clone)]
pub struct LearnedModel {
    pub a0: f64,
    pub a1: f64,
    pub a2: f64,
}

impl DifferentialEquation for LearnedModel {
    fn derivative(&self, _t: f64, y: f64) -> f64 {
        self.a0 + self.a1 * y + self.a2 * y * y
    }
}
