use crate::exercises::logistic_model::parameters::LogisticParams;
use crate::solvers::data::model::Model;

pub struct LogisticAnalytical {
    pub params: LogisticParams,
}

impl LogisticAnalytical {
    pub fn new(params: LogisticParams) -> Self {
        Self { params }
    }

    pub fn solve(&self, t: f64) -> f64 {
        let y0 = self.params.y0;

        let a = (1.0 - y0) / y0;

        1.0 / (1.0 + a * (-t).exp())
    }
}

impl Model for LogisticAnalytical {
    fn evaluate(&self, t: f64) -> f64 {
        let y0 = self.params.y0;
        let a = (1.0 - y0) / y0;

        1.0 / (1.0 + a * (-t).exp())
    }
}