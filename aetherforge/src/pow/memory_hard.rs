use argon2::{self, Config, ThreadMode, Variant, Version};

/// Memory-hard hashing using Argon2.
pub fn memory_hard_hash(input: &[u8], salt: &[u8]) -> Vec<u8> {
    let config = Config {
        variant: Variant::Argon2id,
        version: Version::Version13,
        mem_cost: 65536, // Adjust based on device capabilities
        time_cost: 3,
        lanes: 4,
        thread_mode: ThreadMode::Parallel,
        secret: &[],
        ad: &[],
        hash_length: 32,
    };
    argon2::hash_raw(input, salt, &config).unwrap()
}