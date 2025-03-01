/// Adjusts the mining difficulty based on block times.
pub fn adjust_difficulty(block_times: &[u64], target_time: u64) -> i64 {
    if block_times.is_empty() {
        return 0; // Handle initial case
    }
    let average_block_time: u64 = block_times.iter().sum::<u64>() / block_times.len() as u64;
    if average_block_time > target_time {
        -1 // Decrease difficulty
    } else {
        1 // Increase difficulty
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjust_difficulty() {
        // Test case 1: Empty block times (initial case)
        let empty_times: &[u64] = &[];
        assert_eq!(adjust_difficulty(empty_times, 60), 0);

        // Test case 2: Average block time is greater than target time
        let slow_times = &[70, 80, 75];
        assert_eq!(adjust_difficulty(slow_times, 60), -1);

        // Test case 3: Average block time is less than target time
        let fast_times = &[50, 40, 55];
        assert_eq!(adjust_difficulty(fast_times, 60), 1);

        // Test case 4: Average block time is equal to target time
        let equal_times = &[60, 60, 60];
        assert_eq!(adjust_difficulty(equal_times, 60), 1);

        //test case 5: large numbers.
        let large_times = &[1000000000, 1000000001, 1000000002];
        assert_eq!(adjust_difficulty(large_times, 500000000), -1);

        //test case 6: mixed numbers
        let mixed_times = &[1, 1000000000, 1];
        assert_eq!(adjust_difficulty(mixed_times, 1000000), -1);
    }
}