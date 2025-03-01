/// Validates a validator's stake.
pub fn validate_stake(validator: &str, stake: u64) -> bool {
    stake >= 1000 // Minimum stake requirement
}

/// Calculates the validator's reward based on their stake.
pub fn calculate_reward(stake: u64) -> u64 {
    stake / 100 // 1% reward for simplicity
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_stake() {
        assert_eq!(validate_stake("validator1", 1000), true);
        assert_eq!(validate_stake("validator2", 1500), true);
        assert_eq!(validate_stake("validator3", 999), false);
        assert_eq!(validate_stake("validator4", 0), false);
    }

    #[test]
    fn test_calculate_reward() {
        assert_eq!(calculate_reward(1000), 10);
        assert_eq!(calculate_reward(2500), 25);
        assert_eq!(calculate_reward(999), 9);
        assert_eq!(calculate_reward(0), 0);
        assert_eq!(calculate_reward(1001), 10);
    }
}