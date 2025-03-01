use super::*;

#[test]
fn test_anvil_block() {
    let validator = "test_validator";
    let stake = 1000;
    let block_data = b"test_block";
    let result = pos::anvil_block(validator, stake, block_data);
    assert_eq!(result.len(), 32); // SHA3-256 output is 32 bytes
}