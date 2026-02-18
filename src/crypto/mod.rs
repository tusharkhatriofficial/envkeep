pub mod key;
// Encryption/decryption functions will go here.

use rpassword::read_password;
use std::io::{self, Write};

use crate::errors::DotkeepError;

/// Prompt the user for a password (input is hidden).
pub fn prompt_password(prompt: &str) -> Result<String, DotkeepError> {
    print!("{}", prompt);
    io::stdout()
        .flush()
        .map_err(|e| DotkeepError::EncryptionError(e.to_string()))?;
    let password = read_password()
        .map_err(|e| DotkeepError::EncryptionError(e.to_string()))?;
    Ok(password)
}

/// Prompt for a new master password with confirmation.
pub fn prompt_new_password() -> Result<String, DotkeepError> {
    let password = prompt_password("Enter master password: ")?;

    if password.len() < 8 {
        return Err(DotkeepError::EncryptionError(
            "Password must be at least 8 characters".to_string(),
        ));
    }

    let confirm = prompt_password("Confirm master password: ")?;

    if password != confirm {
        return Err(DotkeepError::EncryptionError(
            "Passwords do not match".to_string(),
        ));
    }

    Ok(password)
}

/// Prompt for the existing master password (single prompt, no confirmation).
pub fn prompt_existing_password() -> Result<String, DotkeepError> {
    prompt_password("Master password: ")
}