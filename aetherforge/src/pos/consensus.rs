use sha3::{Digest, Sha3_256};

/// Finalizes a block using PoS consensus.
pub fn finalize_block(block_data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha3_256::new();
    hasher.update(block_data);
    hasher.finalize().to_vec()
}

/// Selects the next validator based on their stake.
pub fn select_validator(validators: &[(&str, u64)]) -> &str {
    validators
        .iter()
        .max_by_key(|(_, stake)| *stake)
        .map(|(validator, _)| *validator)
        .unwrap_or("default_validator")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_finalize_block() {
        let block_data = b"test block data";
        let hash = finalize_block(block_data);

        // Check that the hash is the correct length (32 bytes for Sha3_256).
        assert_eq!(hash.len(), 32);

        // Verify that the hash is consistent for the same input.
        let hash2 = finalize_block(block_data);
        assert_eq!(hash, hash2);

        //Verify that different inputs produce different outputs.
        let block_data2 = b"different block data";
        let hash3 = finalize_block(block_data2);
        assert_ne!(hash, hash3);
    }

    #[test]
    fn test_select_validator() {
        let validators = vec![("alice", 100), ("bob", 200), ("charlie", 150)];
        let selected_validator = select_validator(&validators);
        assert_eq!(selected_validator, "bob");

        let validators2 = vec![("alice", 100)];
        let selected_validator2 = select_validator(&validators2);
        assert_eq!(selected_validator2, "alice");

        let validators3: Vec<(&str, u64)> = vec![];
        let selected_validator3 = select_validator(&validators3);
        assert_eq!(selected_validator3, "default_validator");
    }
}