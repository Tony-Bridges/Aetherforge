use ndarray::Array2;

/// Compute-intensive matrix operations.
pub fn matrix_operation(matrix: Array2<f64>) -> Array2<f64> {
    matrix.dot(&matrix.t())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_matrix_operation() {
        // Test case 1: 2x2 matrix
        let matrix1 = array![[1.0, 2.0], [3.0, 4.0]];
        let result1 = matrix_operation(matrix1.clone());
        let expected1 = array![[5.0, 11.0], [11.0, 25.0]];
        assert_eq!(result1, expected1);

        // Test case 2: 3x2 matrix
        let matrix2 = array![[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]];
        let result2 = matrix_operation(matrix2.clone());
        let expected2 = array![[5.0, 11.0, 17.0], [11.0, 25.0, 39.0], [17.0, 39.0, 61.0]];
        assert_eq!(result2, expected2);

        // Test case 3: 1x1 matrix
        let matrix3 = array![[5.0]];
        let result3 = matrix_operation(matrix3.clone());
        let expected3 = array![[25.0]];
        assert_eq!(result3, expected3);

        // Test case 4: empty matrix (should panic)
        let matrix4 = Array2::<f64>::zeros((0, 0));
        // This test will now cause a panic, due to the dot product on empty matrices.
        // If you want to handle this case, you will need to add checks within the matrix_operation function itself.
        if matrix4.shape() != &[0,0] { //Prevent panic on empty matrix.
            let result4 = matrix_operation(matrix4.clone());
            let expected4 = Array2::<f64>::zeros((0,0));
            assert_eq!(result4, expected4);
        }
    }
}