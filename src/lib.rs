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
        assert_eq!(
            (matrix::Matrix::new(3, 3, 1.0)).tanh().data,
            vec![0.7615941559557649; 9]
        )
    }

    #[test]
    fn matrix_relu() {
        assert_eq!((matrix::Matrix::new(3, 4, -1.0)).relu().data, vec![0.0; 12])
    }

    #[test]
    fn matrix_sigmoid() {
        //assert_eq!((matrix::Matrix::new(3, 4, -1.0)).sigmoid().data, vec![0.0; 12])
        let a = (matrix::Matrix::new(3, 4, 0.0)).sigmoid().data;
        println!("content {:?}", a);
        println!("a printed!");
    }
}
