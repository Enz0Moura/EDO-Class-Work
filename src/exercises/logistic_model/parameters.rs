#[derive(Clone)]
pub struct LogisticParams {
    pub y0: f64,
}

impl LogisticParams {
    pub fn new(y0: f64) -> Self {
        Self { y0 }
    }
}
