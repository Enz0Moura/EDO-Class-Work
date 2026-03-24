pub struct LogisticLeastSquaresBuilder;

pub struct LeastSquaresProblem {
    pub A: Vec<Vec<f64>>,
    pub r: Vec<f64>,
}

impl LogisticLeastSquaresBuilder {
    pub fn build(data: &[(f64, f64)], h: f64) -> LeastSquaresProblem {
        let mut A = Vec::new();
        let mut r = Vec::new();

        for i in 0..data.len() - 1 {
            let y_i = data[i].1;
            let y_next = data[i + 1].1;

            let ri = (y_next - y_i) / h;

            A.push(vec![1.0, y_i, y_i * y_i]);
            r.push(ri);
        }

        LeastSquaresProblem { A, r }
    }
}
