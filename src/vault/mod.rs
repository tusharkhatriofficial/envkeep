pub mod project;
pub mod variable;

use rusqlite::Connection;
use std::path::PathBuf;

use crate::errors::EnvkeepError;
use crate::crypto::key::generate_salt;

///Get the path to envkeep data directory
pub fn data_dir() -> PathBuf {
    let mut path = dirs::home_dir().expect("Could not find home directory");
    path.push(".envkeep");
    path
}

///Get the path to the vault database (~/.envkeep/vault.db)
pub fn vault_path() -> PathBuf {
    let mut path = data_dir();
    path.push("vault.db");
    path
}

///Check if the vault exists.
pub fn vault_exists() -> bool {
    vault_path().exists()
}

///open the vault database with the master password
///
/// This sets the SQLCipher encryption ket and returns a connection.
pub fn open_vault(password: &str) -> Result<Connection, EnvkeepError>{
    let path = vault_path();

    if !path.exists() {
        return Err(EnvkeepError::VaultNotFound);
    }

    let conn = Connection::open(&path)?;

    //Set the sql cipher encryption key
    conn.pragma_update(None, "key", &password)?;

    //Test that the key is correct by querying the schema
    conn.execute_batch("SELECT count(*) FROM sqlite_master;")
        .map_err(|_| EnvkeepError::WrongPassword)?;

    Ok(conn)
}


///create a new vault database with the master password
pub fn create_vault(password: &str) -> Result<Connection, EnvkeepError> {
    let dir = data_dir();
    let path = vault_path();

    if path.exists() {
        return Err(EnvkeepError::VaultAlreadyExists(
            path.display().to_string(),
        ));
    }

    //create the dir if it does not exists
    std::fs::create_dir_all(&dir)
        .map_err(|e| EnvkeepError::FileWriteError(dir.display().to_string(), e))?;

    let conn = Connection::open(&path)?;

    //Set the sql cipher encryption key
    conn.pragma_update(None, "key", &password)?;

    // Run Schema mirations
    run_migrations(&conn)?;

    let salt = generate_salt()?;
    let salt_hex: String = salt.iter().map(|b| format!("{:02x}", b)).collect();
    conn.execute(
        "INSERT INTO metadata (key, value) VALUES ('salt', ?1)",
        [&salt_hex],
    )?;

    // verification hash so we can verify the password later
    let verification_key = crate::crypto::key::derive_key(password, &salt);
    let verification_hex: String = verification_key.iter().map(|b| format!("{:02x}", b)).collect();
    conn.execute(
        "INSERT INTO metadata (key, value) VALUES ('verification_key', ?1)",
        [&verification_hex],
    )?;

    Ok(conn)

}

/// Get the encryption key by reading the salt from the database and deriving the key.
pub fn get_encryption_key(conn: &Connection, password: &str) -> Result<[u8; 32], EnvkeepError> {
    let salt_hex: String = conn.query_row(
        "SELECT value FROM metadata WHERE key = 'salt'",
        [],
        |row| row.get(0),
    )?;

    let salt: Vec<u8> = (0..salt_hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&salt_hex[i..i + 2], 16).unwrap())
        .collect();

    Ok(crate::crypto::key::derive_key(password, &salt))
}


///run all the database migrations
fn run_migrations(conn: &Connection) -> Result<(), EnvkeepError> {
    conn.execute_batch(
        "
        -- Metadata table for vault configuration
        CREATE TABLE IF NOT EXISTS metadata (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );

        -- Projects table
        CREATE TABLE IF NOT EXISTS projects (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            directory TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            last_used_at TEXT
        );

        -- Variables table (encrypted values)
        CREATE TABLE IF NOT EXISTS variables (
            id TEXT PRIMARY KEY,
            project_id TEXT NOT NULL,
            key TEXT NOT NULL,
            encrypted_value TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
            UNIQUE(project_id, key)
        );

        -- Shared secrets table
        CREATE TABLE IF NOT EXISTS secrets (
            id TEXT PRIMARY KEY,
            key TEXT NOT NULL UNIQUE,
            encrypted_value TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );

        -- Secret-to-project links
        CREATE TABLE IF NOT EXISTS secret_links (
            secret_id TEXT NOT NULL,
            project_id TEXT NOT NULL,
            PRIMARY KEY (secret_id, project_id),
            FOREIGN KEY (secret_id) REFERENCES secrets(id) ON DELETE CASCADE,
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
        );

        -- Store the key derivation salt
        INSERT OR IGNORE INTO metadata (key, value)
        VALUES ('schema_version', '1');
        ",
    )?;

    Ok(())
}


pub use project::Project;
pub use variable::Variable;