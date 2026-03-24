use crate::exercises::logistic_model::parameters::LogisticParams;
use crate::solvers::differetials;

#[derive(Clone)]
pub struct LogisticDifferential {
    pub params: LogisticParams,
}

impl LogisticDifferential {
    pub fn new(params: LogisticParams) -> Self {
        Self { params }
    }


}

impl differetials::DifferentialEquation for LogisticDifferential {
    fn derivative(&self, _t: f64, y: f64) -> f64 {
        y * (1.0 - y)
    }
}