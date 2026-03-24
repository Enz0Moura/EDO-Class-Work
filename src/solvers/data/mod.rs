use rand_distr::{Normal, Distribution};
use crate::solvers::data::model::Model;
pub mod model;


pub struct DataGenerator<M: Model> {
    pub model: M,
}

impl<M: Model> DataGenerator<M> {
    pub fn new(model: M) -> Self {
        Self { model }
    }

    pub fn generate(&self, t_values: &[f64], noise_std: f64) -> Vec<(f64, f64)> {
        let normal = Normal::new(0.0, noise_std).unwrap();
        let mut rng = rand::thread_rng();

        t_values
            .iter()
            .map(|&t| {
                let y = self.model.evaluate(t);
                let noise = normal.sample(&mut rng);
                (t, y + noise)
            })
            .collect()
    }
}

