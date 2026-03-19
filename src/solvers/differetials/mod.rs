pub trait DifferentialEquation {
    fn derivative(&self, t: f64, y: f64) -> f64;
}
