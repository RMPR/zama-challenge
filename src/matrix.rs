use std::ops;

pub struct Matrix {
    n: usize,
    m: usize,
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(n: usize, m: usize, e: f64) -> Matrix {
        let matrix = Matrix {
            n: n,
            m: m,
            data: vec![vec![e; n]; m],
        };
        return matrix;
    }
}
impl ops::Add<Matrix> for Matrix {
    type Output = Self;

    fn add(self, rhs: Matrix) -> Matrix {
        if self.n != rhs.n || self.m != rhs.m {
            panic!("Incompatible sizes");
        }

        let mut matrix_sum = Matrix::new(self.n, self.m, 0.0);
        for i in 0..self.n {
            for j in 0..self.m {
                matrix_sum.data[i][j] = self.data[i][j] + rhs.data[i][j]
            }
        }
        return matrix_sum;
    }
}
