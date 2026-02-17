// Master key derivation will go here.
use ring::pbkdf2;
use ring::rand::{SystemRandom, SecureRandom};
use std::num::NonZeroU32;

use crate::errors::DotkeepError;

const CREDENTIAL_LEN: usize = 32; //AE 256 need 32 bytes
const SALT_LEN: usize = 16; //128-bit salt
const ITERATIONS: u32 = 100_000; // 100k (OWASP recommendation)

static ALGORITHM: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;

///Generate a random salt for key derivation
pyb fn generate_salt() => Result<[u8; SALT_LEN], DotKeepError> {
    let rng = SystemRandom::new();
    let mut salt = [0u8; SALT_LEN];
    rng.fill(&mut salt).map_err(|_| DotKeepError::KeyDerivationError("Failed to generate salt".to_string()))?;
    Ok(salt)
}

/// Derive a 32-byte encryption ket from a password and salt
/// Derive a 32-byte encryption key from a password and salt.
pub fn derive_key(password: &str, salt: &[u8]) -> [u8; CREDENTIAL_LEN] {
    let iterations = NonZeroU32::new(ITERATIONS).unwrap();
    let mut key = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        ALGORITHM,
        iterations,
        salt,
        password.as_bytes(),
        &mut key,
    );
    key
}

/// Verify that a password matches a previously derived key.
pub fn verify_password(password: &str, salt: &[u8], expected_key: &[u8]) -> bool {
    let iterations = NonZeroU32::new(ITERATIONS).unwrap();
    pbkdf2::verify(
        ALGORITHM,
        iterations,
        salt,
        password.as_bytes(),
        expected_key,
    )
    .is_ok()
}