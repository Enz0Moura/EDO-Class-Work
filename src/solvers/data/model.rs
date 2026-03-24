pub trait Model {
    fn evaluate(&self, t: f64) -> f64;
}
