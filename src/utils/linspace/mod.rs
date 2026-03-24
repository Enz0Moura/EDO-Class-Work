pub struct Linspace {
    start: f64,
    end: f64,
    n: usize,
    step: f64,
}

impl Linspace {
    pub fn new(start: f64, end: f64, n: usize) -> Self {
        assert!(n >= 2, "Linspace needs at least 2 points");

        let step = (end - start) / (n as f64 - 1.0);

        Self {
            start,
            end,
            n,
            step,
        }
    }

    pub fn generate(&self) -> Vec<f64> {
        (0..self.n)
            .map(|i| self.start + i as f64 * self.step)
            .collect()
    }
    pub fn iter(&self) -> impl Iterator<Item = f64> + '_ {
        (0..self.n).map(|i| self.start + i as f64 * self.step)
    }
    pub fn get(&self, i: usize) -> Option<f64> {
        if i < self.n {
            Some(self.start + i as f64 * self.step)
        } else {
            None
        }
    }

    pub fn step(&self) -> f64 {
        self.step
    }
}