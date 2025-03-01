pub mod staking;
pub mod consensus;

/// Combines staking and consensus logic.
pub fn anvil_block(validator: &str, stake: u64, block_data: &[u8]) -> Vec<u8> {
    if staking::validate_stake(validator, stake) {
        consensus::finalize_block(block_data)
    } else {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock implementations for testing
    mod staking {
        pub fn validate_stake(validator: &str, stake: u64) -> bool {
            stake >= 1000 // Mock minimum stake
        }
    }

    mod consensus {
        pub fn finalize_block(block_data: &[u8]) -> Vec<u8> {
            // Mock consensus: return the input data with a byte appended.
            let mut result = block_data.to_vec();
            result.push(42);
            result
        }
    }

    #[test]
    fn test_anvil_block() {
        let validator = "test_validator";
        let valid_stake = 1500;
        let invalid_stake = 500;
        let block_data = b"test_block_data";

        // Test with valid stake
        let result_valid = anvil_block(validator, valid_stake, block_data);
        let mut expected_valid = block_data.to_vec();
        expected_valid.push(42);
        assert_eq!(result_valid, expected_valid);

        // Test with invalid stake
        let result_invalid = anvil_block(validator, invalid_stake, block_data);
        assert_eq!(result_invalid, Vec::new());
    }
}