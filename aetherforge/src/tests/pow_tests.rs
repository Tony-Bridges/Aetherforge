use super::*;

#[test]
fn test_forge_block() {
    let input = b"test_input";
    let salt = b"test_salt";
    let matrix = ndarray::Array2::zeros((2, 2));
    let puzzle_data = b"test_puzzle";
    let result = pow::forge_block(input, salt, matrix, puzzle_data);
    assert_eq!(result.len(), 32); // SHA3-256 output is 32 bytes
}