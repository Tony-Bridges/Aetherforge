#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_hard_hash() {
        let input = b"test_input";
        let salt = b"test_salt";
        let hash = memory_hard_hash(input, salt);
        assert_eq!(hash.len(), 32); // SHA3-256 output is 32 bytes
    }
}