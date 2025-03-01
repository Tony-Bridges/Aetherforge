pub mod memory_hard;
pub mod matrix_ops;
pub mod puzzles;

use ndarray::Array2;

/// Combines memory-hard hashing, matrix operations, and puzzle solving.
pub fn forge_block(input: &[u8], salt: &[u8], matrix: Array2<f64>, puzzle_data: &[u8]) -> Vec<u8> {
    let memory_hash = memory_hard::memory_hard_hash(input, salt);
    let matrix_result = matrix_ops::matrix_operation(matrix);
    let puzzle_result = puzzles::solve_puzzle(puzzle_data);

    let mut combined_input = Vec::new();
    combined_input.extend_from_slice(&memory_hash);
    combined_input.extend_from_slice(matrix_result.as_slice().unwrap());
    combined_input.extend_from_slice(&puzzle_result);

    let mut hasher = sha3::Sha3_256::new();
    hasher.update(combined_input);
    hasher.finalize().to_vec()
}
