/// This module contains tests
mod matrix;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_creation() {
        let m = matrix::Matrix::new(3, 3, 0.0);
        assert_eq!(m.data, vec![0.0f64; 9]);
    }

    #[test]
    fn matrix_sum() {
        assert_eq!(
            (matrix::Matrix::new(3, 3, 2.0) + matrix::Matrix::new(3, 3, 1.0)).data,
            vec![3.0; 9]
        )
    }

    #[test]
    fn matrix_scalar_mul() {
        assert_eq!(
            (matrix::Scalar::new(4.0) * matrix::Matrix::new(3, 3, 1.0)).data,
            vec![4.0; 9]
        );
    }

    #[test]
    fn matrix_tanh() {
        let mut m = matrix::Matrix::new(3, 3, 1.0);
        m.tanh();
        assert_eq!(m.data, vec![0.7615941559557649; 9])
    }

    #[test]
    fn matrix_relu() {
        let mut m = matrix::Matrix::new(3, 4, -1.0);
        m.relu();
        assert_eq!(m.data, vec![0.0; 12]);
    }

    #[test]
    fn matrix_sigmoid() {
        let mut m = matrix::Matrix::new(4, 3, 0.0);
        m.sigmoid();
        assert_eq!(m.data, vec![0.5; 12]);
    }

    #[test]
    fn matrix_flatten() {
        let mut m = matrix::Matrix::new(4, 3, 1.0);
        m.flatten();
        assert_eq!(m.data, vec![1.0; 12]);
        assert_eq!(m.n, 1);
        assert_eq!(m.m, 12);
    }

    #[test]
    fn matrix_mul() {
        let mut m = matrix::Matrix::new(3, 3, 0.0);
        let mut v = matrix::Matrix::new(3, 1, 0.0);
        m.data = [1.0, 5.0, 7.0, 3.0, 8.0, 9.0, 4.0, 6.0, 2.0].to_vec();
        v.data = [11.0, 12.0, 13.0].to_vec();

        assert_eq!((m * v).data, vec![162.0, 246.0, 142.0]);
    }
}
