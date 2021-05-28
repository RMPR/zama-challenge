pub struct Matrix {
    n: usize,
    m: usize,
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(n: usize, m: usize) -> Matrix {
        let matrix = Matrix {
            n: n,
            m: m,
            data: vec![vec![0.0f64; n]; m],
        };
        return matrix;
    }
}
