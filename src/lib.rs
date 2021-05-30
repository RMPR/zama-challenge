mod matrix;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn matrix_creation() {
        let m = matrix::Matrix::new(3, 3, 0.0);
        assert_eq!(m.data, vec![vec![0.0f64; 3]; 3]);
    }

    #[test]
    fn matrix_sum() {
        assert_eq!((matrix::Matrix::new(3, 3, 2.0) + matrix::Matrix::new(3, 3, 1.0)).data, vec![vec![3.0; 3]; 3])
    }
}
