use std::ops::{Add, Mul};

pub struct Scalar {
    value: f64,
}

pub struct Matrix {
    n: usize,
    m: usize,
    pub data: Vec<f64>,
}

impl Matrix {
    pub fn new(n: usize, m: usize, e: f64) -> Matrix {
        let matrix = Matrix {
            n,
            m,
            data: vec![e; n * m],
        };
        return matrix;
    }

    pub fn get(i: usize, j: usize) -> f64 {
        0.0
    }
}

impl Scalar {
    pub fn new(value: f64) -> Scalar {
        Scalar {
            value,
        }
    }
}

impl Add<Matrix> for Matrix {
    type Output = Self;

    fn add(self, rhs: Matrix) -> Matrix {
        if self.n != rhs.n || self.m != rhs.m {
            panic!("Incompatible sizes");
        }

        Matrix {
            n: self.n,
            m: self.m,
            data: (0..self.n * self.m)
                .map(|i| self.data[i] + rhs.data[i])
                .collect(),
        }
    }
}

impl Mul<Matrix> for Scalar {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Matrix {
        Matrix {
            n: rhs.n,
            m: rhs.m,
            data: rhs.data.iter().map(|v| v * self.value as f64).collect(),
        }
    }
}
