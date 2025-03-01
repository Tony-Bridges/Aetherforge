use sha3::{Digest, Sha3_256};
use rhai::{Engine, EvalAltResult, Scope, Position};
use thiserror::Error; // For custom error handling
use std::time::{Duration, Instant};

/// Custom error type for coding challenge
#[derive(Error, Debug)]
pub enum CodingChallengeError {
    #[error("Code evaluation failed: {0}")]
    RhaiError(#[from] Box<EvalAltResult>),
    #[error("Result does not match expected value")]
    ResultMismatch,
    #[error("Code execution timed out")]
    Timeout,
    #[error("Invalid code syntax")]
    SyntaxError,
}

/// Solves a coding challenge by evaluating the provided code and checking the result.
pub fn solve(code: &[u8], expected_result: i64) -> Result<Vec<u8>, CodingChallengeError> {
    // 1. Convert code to string
    let code_str = String::from_utf8_lossy(code).into_owned();

    // 2. Create Rhai engine
    let engine = Engine::new();

    // 3. Set a timeout of 1 second
    let start_time = Instant::now();
    let timeout = Duration::from_secs(1);

    // 4. Evaluate the code
    let result: i64 = match engine.eval_with_scope(&mut Scope::new(), &code_str) {
        Ok(result) => result,
        Err(err) => {
            // Check for syntax errors
            if err.to_string().contains("syntax error") {
                return Err(CodingChallengeError::SyntaxError);
            }
            return Err(CodingChallengeError::RhaiError(err));
        }
    };

    // 5. Check for timeout
    if start_time.elapsed() > timeout {
        return Err(CodingChallengeError::Timeout);
    }

    // 6. Check if the result matches the expected value
    if result == expected_result {
        // 7. Hash the code if the result is correct
        let mut hasher = Sha3_256::new();
        hasher.update(code);
        let hash = hasher.finalize().to_vec();
        Ok(hash)
    } else {
        Err(CodingChallengeError::ResultMismatch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_correct_code() {
        // Test with correct code
        let code = b"let x = 40; x + 2";
        let expected_result = 42;
        let result = solve(code, expected_result);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 32); // SHA3-256 output is 32 bytes
    }

    #[test]
    fn test_solve_incorrect_code() {
        // Test with incorrect code
        let code = b"let x = 41; x + 1";
        let expected_result = 42;
        let result = solve(code, expected_result);
        assert!(matches!(result, Err(CodingChallengeError::ResultMismatch)));
    }

    #[test]
    fn test_solve_invalid_code() {
        // Test with invalid code
        let code = b"invalid code";
        let expected_result = 42;
        let result = solve(code, expected_result);
        assert!(matches!(result, Err(CodingChallengeError::SyntaxError)));
    }

    #[test]
    fn test_solve_timeout() {
        // Test with code that takes too long to execute
        let code = b"let mut sum = 0; for i in 0..100000000 { sum += i; } sum";
        let expected_result = 0; // The result doesn't matter in this case
        let result = solve(code, expected_result);
        assert!(matches!(result, Err(CodingChallengeError::Timeout)));
    }
}