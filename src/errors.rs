use thiserror::Error;

#[derive(Error, Debug)]
pub enum DotkeepError {
    // Vault errors
    #[error("Vault not found. Run 'dotkeep init' first.")]
    VaultNotFound,

    #[error("Vault already exists at {0}")]
    VaultAlreadyExists(String),

    #[error("Failed to open vault database: {0}")]
    DatabaseError(#[from] rusqlite::Error),

    // Crypto errors
    #[error("Wrong master password")]
    WrongPassword,

    #[error("Encryption failed: {0}")]
    EncryptionError(String),

    #[error("Decryption failed: {0}")]
    DecryptionError(String),

    #[error("Key derivation failed: {0}")]
    KeyDerivationError(String),

    // Project errors
    #[error("Project '{0}' not found")]
    ProjectNotFound(String),

    #[error("Project '{0}' already exists")]
    ProjectAlreadyExists(String),

    // File errors
    #[error("No .env file found in current directory")]
    NoEnvFile,

    #[error("Failed to read file '{0}': {1}")]
    FileReadError(String, std::io::Error),

    #[error("Failed to write file '{0}': {1}")]
    FileWriteError(String, std::io::Error),

    // Secret errors
    #[error("Secret '{0}' not found")]
    SecretNotFound(String),

    #[error("Invalid key=value format: '{0}'")]
    InvalidKeyValue(String),

    // Export/Import errors
    #[error("Invalid vault file format")]
    InvalidVaultFile,

    #[error("Backup file not found: {0}")]
    BackupNotFound(String),
}