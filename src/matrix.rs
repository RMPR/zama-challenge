use std::ops::{Add, Mul};

pub struct Scalar {
    value: f64,
}

pub struct Matrix {
    pub n: usize,
    pub m: usize,
    pub data: Vec<f64>,
}

impl Matrix {
    /// Create a new Matrix initializing all elements with e
    pub fn new(n: usize, m: usize, e: f64) -> Matrix {
        let matrix = Matrix {
            n,
            m,
            data: vec![e; n * m],
        };
        return matrix;
    }

    /// Return the element at line i and column j
    pub fn get(&mut self, i: usize, j: usize) -> f64 {
        return self.data[self.n * i + j];
    }

    pub fn set(&mut self, i: usize, j: usize, value: f64) {
        self.data[self.n * i + j] = value;
    }

    /// Apply tanh on each element of the Matrix and return a new one
    pub fn tanh(&mut self) {
        self.data = self.data.iter().map(|v| v.tanh()).collect();
    }

    /// Apply relU on each element of the Matrix and return a new one
    pub fn relu(&mut self) {
        self.data = self.data.iter().map(|v| v.max(0.0)).collect();
    }

    /// Apply sigmoid on each element of the Matrix and return a new one
    pub fn sigmoid(&mut self) {
        self.data = self.data.iter().map(|v| 1.0 / (1.0 + (-v).exp())).collect();
    }

    pub fn flatten(&mut self) {
        self.m *= self.n;
        self.n = 1;
    }
}

impl Scalar {
    pub fn new(value: f64) -> Scalar {
        Scalar { value }
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

impl Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(mut self, mut rhs: Matrix) -> Matrix {
        // TODO; Use a more efficient algorithm
        if self.m != rhs.n {
            panic!("Size mismatch");
        }
        let mut res = Matrix::new(self.n, rhs.m, 0.0);
        for i in 0..self.n {
            for j in 0..rhs.m {
                let mut sum: f64 = 0.0;
                for k in 0..rhs.n {
                    sum += self.get(i, k) * rhs.get(j, k);
                }
                res.set(j, i, sum);
            }
        }
        return res;
    }
}
