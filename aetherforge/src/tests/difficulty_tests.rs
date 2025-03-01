use super::*;

#[test]
fn test_adjust_difficulty() {
    let block_times = vec![10, 20, 30];
    let target_time = 20;
    let result = difficulty::adjust_difficulty(&block_times, target_time);
    assert!(result == -1 || result == 1);
}