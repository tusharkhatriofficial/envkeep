pub mod key;
// Encryption/decryption functions will go here.

use ring::aead::{self, Aad, BoundKey, NONCE_LEN, Nonce, NonceSequence, UnboundKey};
use ring::error::Unspecified;
use ring::rand::{SecureRandom, SystemRandom};
use rpassword::read_password;
use std::io::{self, Write};

use crate::errors::EnvkeepError;

/// Prompt the user for a password (input is hidden).
pub fn prompt_password(prompt: &str) -> Result<String, EnvkeepError> {
    print!("{}", prompt);
    io::stdout()
        .flush()
        .map_err(|e| EnvkeepError::EncryptionError(e.to_string()))?;
    let password = read_password().map_err(|e| EnvkeepError::EncryptionError(e.to_string()))?;
    Ok(password)
}

/// Prompt for a new master password with confirmation.
pub fn prompt_new_password() -> Result<String, EnvkeepError> {
    let password = prompt_password("Enter master password: ")?;

    if password.len() < 8 {
        return Err(EnvkeepError::EncryptionError(
            "Password must be at least 8 characters".to_string(),
        ));
    }

    let confirm = prompt_password("Confirm master password: ")?;

    if password != confirm {
        return Err(EnvkeepError::EncryptionError(
            "Passwords do not match".to_string(),
        ));
    }

    Ok(password)
}

/// Prompt for the existing master password (single prompt, no confirmation).
pub fn prompt_existing_password() -> Result<String, EnvkeepError> {
    prompt_password("Master password: ")
}

const TAG_LEN: usize = 16; // AES-GCM authentication tag length

///A nonce sequence that uses a single pre-generated nonce.
struct SingleNonce {
    nonce: Option<[u8; NONCE_LEN]>,
}

impl SingleNonce {
    fn new(nonce: [u8; NONCE_LEN]) -> Self {
        Self { nonce: Some(nonce) }
    }
}

impl NonceSequence for SingleNonce {
    fn advance(&mut self) -> Result<Nonce, Unspecified> {
        self.nonce
            .take()
            .map(Nonce::assume_unique_for_key)
            .ok_or(Unspecified)
    }
}

/// Encrypt plaintext bytes using AES-256-GCM.
///
/// Returns: nonce (12 bytes) + ciphertext + tag (16 bytes)
pub fn encrypt(key: &[u8; 32], plaintext: &[u8]) -> Result<Vec<u8>, EnvkeepError> {
    let rng = SystemRandom::new();

    // Generate a random 12-byte nonce
    let mut nonce_bytes = [0u8; NONCE_LEN];
    rng.fill(&mut nonce_bytes)
        .map_err(|_| EnvkeepError::EncryptionError("Failed to generate nonce".to_string()))?;

    // Create the sealing key
    let unbound_key = UnboundKey::new(&aead::AES_256_GCM, key)
        .map_err(|_| EnvkeepError::EncryptionError("Invalid key".to_string()))?;
    let mut sealing_key = aead::SealingKey::new(unbound_key, SingleNonce::new(nonce_bytes));

    // Encrypt in place
    let mut in_out = plaintext.to_vec();
    sealing_key
        .seal_in_place_append_tag(Aad::empty(), &mut in_out)
        .map_err(|_| EnvkeepError::EncryptionError("Encryption failed".to_string()))?;

    // Prepend nonce to ciphertext: [nonce (12) | ciphertext | tag (16)]
    let mut result = Vec::with_capacity(NONCE_LEN + in_out.len());
    result.extend_from_slice(&nonce_bytes);
    result.extend_from_slice(&in_out);

    Ok(result)
}

/// Decrypt ciphertext produced by encrypt().
///
/// Input format: nonce (12 bytes) + ciphertext + tag (16 bytes)
pub fn decrypt(key: &[u8; 32], encrypted: &[u8]) -> Result<Vec<u8>, EnvkeepError> {
    if encrypted.len() < NONCE_LEN + TAG_LEN {
        return Err(EnvkeepError::DecryptionError(
            "Ciphertext too short".to_string(),
        ));
    }

    // Split nonce from ciphertext
    let (nonce_bytes, ciphertext_with_tag) = encrypted.split_at(NONCE_LEN);
    let mut nonce_arr = [0u8; NONCE_LEN];
    nonce_arr.copy_from_slice(nonce_bytes);

    // Create the opening key
    let unbound_key = UnboundKey::new(&aead::AES_256_GCM, key)
        .map_err(|_| EnvkeepError::DecryptionError("Invalid key".to_string()))?;
    let mut opening_key = aead::OpeningKey::new(unbound_key, SingleNonce::new(nonce_arr));

    // Decrypt in place
    let mut in_out = ciphertext_with_tag.to_vec();
    let plaintext = opening_key
        .open_in_place(Aad::empty(), &mut in_out)
        .map_err(|_| {
            EnvkeepError::DecryptionError(
                "Decryption failed (wrong password or corrupted data)".to_string(),
            )
        })?;

    Ok(plaintext.to_vec())
}

// helper functions

/// Encrypt a string value. Returns base64-encoded ciphertext.
pub fn encrypt_value(key: &[u8; 32], value: &str) -> Result<String, EnvkeepError> {
    let encrypted = encrypt(key, value.as_bytes())?;
    Ok(base64_encode(&encrypted))
}

/// Decrypt a base64-encoded ciphertext back to a string.
pub fn decrypt_value(key: &[u8; 32], encoded: &str) -> Result<String, EnvkeepError> {
    let encrypted = base64_decode(encoded)?;
    let decrypted = decrypt(key, &encrypted)?;
    String::from_utf8(decrypted).map_err(|e| EnvkeepError::DecryptionError(e.to_string()))
}

// Simple base64 encoding/decoding (no external dependency needed)
fn base64_encode(data: &[u8]) -> String {
    use ring::test::rand::FixedByteRandom; // not this
    // Actually, use the `base64` crate or manual impl.
    // For simplicity, store as hex:
    data.iter().map(|b| format!("{:02x}", b)).collect()
}

fn base64_decode(hex: &str) -> Result<Vec<u8>, EnvkeepError> {
    (0..hex.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&hex[i..i + 2], 16)
                .map_err(|_| EnvkeepError::DecryptionError("Invalid hex".to_string()))
        })
        .collect()
}
