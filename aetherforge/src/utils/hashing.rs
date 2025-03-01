use sha3::{Digest, Sha3_256};

/// Generates a SHA3-256 hash of the input data.
pub fn sha3_256_hash(input: &[u8]) -> Vec<u8> {
    let mut hasher = Sha3_256::new();
    hasher.update(input);
    hasher.finalize().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha3_256_hash() {
        let input = b"hello world";
        let hash = sha3_256_hash(input);

        // Check the hash length.
        assert_eq!(hash.len(), 32);

        // Verify hash consistency.
        let hash2 = sha3_256_hash(input);
        assert_eq!(hash, hash2);

        // Verify different inputs produce different hashes.
        let input2 = b"hello world!";
        let hash3 = sha3_256_hash(input2);
        assert_ne!(hash, hash3);

        // Test with empty input
        let empty_input = b"";
        let empty_hash = sha3_256_hash(empty_input);
        assert_eq!(empty_hash.len(), 32);
    }
}