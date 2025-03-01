use elliptic_curve::sec1::ToEncodedPoint;
use k256::{Secp256k1, SecretKey};
use rand_core::OsRng;

/// Generates a new ECDSA key pair.
pub fn generate_keypair() -> (SecretKey, Vec<u8>) {
    let secret_key = SecretKey::random(&mut OsRng);
    let public_key = secret_key.public_key().to_encoded_point(false);
    (secret_key, public_key.as_bytes().to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    use k256::PublicKey;
    use elliptic_curve::sec1::FromEncodedPoint;

    #[test]
    fn test_generate_keypair() {
        let (secret_key, public_key_bytes) = generate_keypair();

        // Check that the secret key is valid.
        assert!(secret_key.verify_public_key().is_ok());

        // Check that the public key bytes are the correct length (uncompressed).
        assert_eq!(public_key_bytes.len(), 65);

        // Check that the public key bytes can be converted back to a public key.
        let public_key_result = PublicKey::from_encoded_point(
            &k256::EncodedPoint::from_bytes(public_key_bytes.clone()).unwrap()
        );

        assert!(public_key_result.is_ok());

        // Check that the derived public key matches the one from the secret key.
        let derived_public_key = secret_key.public_key();
        assert_eq!(derived_public_key, public_key_result.unwrap());
    }
}