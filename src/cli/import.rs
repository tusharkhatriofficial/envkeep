use anyhow::{Context, Result};
use colored::Colorize;
use std::fs;
use std::path::Path;

use crate::crypto;
use crate::errors::EnvkeepError;
use crate::vault;
use crate::vault::project::Project;

use super::export::MAGIC; // re-export MAGIC from export module

// Note: you may need to make MAGIC pub in export.rs

pub fn handle_import(file_path: &str) -> Result<()> {
    let path = Path::new(file_path);
    if !path.exists() {
        return Err(EnvkeepError::BackupNotFound(file_path.to_string()).into());
    }

    // Read the file
    let data = fs::read(path)
        .map_err(|e| EnvkeepError::FileReadError(file_path.to_string(), e))?;

    // Verify magic bytes
    if data.len() < MAGIC.len() + 16 || &data[..MAGIC.len()] != MAGIC {
        return Err(EnvkeepError::InvalidVaultFile.into());
    }

    // Extract salt and encrypted data
    let salt = &data[MAGIC.len()..MAGIC.len() + 16];
    let encrypted = &data[MAGIC.len() + 16..];

    // Ask for export passphrase
    let export_password = crypto::prompt_password("Export passphrase: ")?;
    let export_key = crypto::key::derive_key(&export_password, salt);

    // Decrypt
    let json_bytes = crypto::decrypt(&export_key, encrypted)
        .context("Wrong passphrase or corrupted file")?;
    let json_str = String::from_utf8(json_bytes)
        .context("Invalid UTF-8 in decrypted data")?;

    // Parse
    let export_data: super::export::ExportData = serde_json::from_str(&json_str)
        .context("Invalid export data format")?;

    // Open the vault
    let vault_password = crypto::prompt_existing_password()?;
    let conn = vault::open_vault(&vault_password)?;
    let enc_key = vault::get_encryption_key(&conn, &vault_password)?;

    // Create the project
    let project = Project::new(&export_data.project_name, None);
    vault::project::create_project(&conn, &project)
        .context(format!(
            "Failed to create project '{}' (already exists?)",
            export_data.project_name
        ))?;

    // Encrypt and store each variable
    let mut count = 0;
    for (key, value) in &export_data.variables {
        let encrypted_value = crypto::encrypt_value(&enc_key, value)?;
        vault::variable::upsert_variable(&conn, &project.id, key, &encrypted_value)?;
        count += 1;
    }

    println!(
        "{} Imported {} ({} variables) from {}",
        "Done.".green().bold(),
        export_data.project_name.cyan(),
        count,
        file_path.bold()
    );

    Ok(())
}