use nalgebra::{DMatrix, DVector};

pub struct LeastSquaresSolver;

impl LeastSquaresSolver {
    pub fn solve(A: &[Vec<f64>], r: &[f64]) -> Vec<f64> {
        let rows = A.len();
        let cols = A[0].len();

        let mut mat = DMatrix::zeros(rows, cols);
        let mut vec = DVector::zeros(rows);

        for i in 0..rows {
            for j in 0..cols {
                mat[(i, j)] = A[i][j];
            }
            vec[i] = r[i];
        }

        let result = mat.svd(true, true).solve(&vec, 1e-8).unwrap();

        result.iter().cloned().collect()
    }
}
