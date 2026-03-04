use anyhow::{Context, Result};
use colored::Colorize;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use std::fs;

use crate::crypto;
use crate::vault;

pub const MAGIC: &[u8] = b"DKVAULT\0";

#[derive(Serialize, Deserialize)]
pub struct ExportData {
    pub version: u32,
    pub project_name: String,
    pub created_at: String,
    pub variables: BTreeMap<String, String>,
}


pub fn handle_export(project_name: &str) -> Result<()> {
    let password = crypto::prompt_existing_password()?;
    let conn = vault::open_vault(&password)?;
    let enc_key = vault::get_encryption_key(&conn, &password)?;

    let project = vault::project::get_project(&conn, project_name)?;
    let variables = vault::variable::get_variables(&conn, &project.id)?;

    if variables.is_empty() {
        println!("Project {} has no variables.", project_name.cyan());
        return Ok(());
    }

    // Decrypt all variables to plaintext
    let mut vars = BTreeMap::new();
    for var in &variables {
        let decrypted = crypto::decrypt_value(&enc_key, &var.encrypted_value)?;
        vars.insert(var.key.clone(), decrypted);
    }

    // Build export data
    let export_data = ExportData {
        version: 1,
        project_name: project_name.to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
        variables: vars,
    };

    let json = serde_json::to_string_pretty(&export_data)?;

    // Ask for export passphrase
    println!("Set a passphrase for the export file.");
    println!("{}", "Share this passphrase with the recipient separately.".dimmed());
    let export_password = crypto::prompt_new_password()
        .context("Failed to read export passphrase")?;

    // Derive key from export passphrase
    let salt = crypto::key::generate_salt()?;
    let export_key = crypto::key::derive_key(&export_password, &salt);

    // Encrypt the JSON
    let encrypted = crypto::encrypt(&export_key, json.as_bytes())?;

    // Build the output file: magic + salt + encrypted
    let mut output = Vec::new();
    output.extend_from_slice(MAGIC);
    output.extend_from_slice(&salt);
    output.extend_from_slice(&encrypted);

    // Write to file
    let filename = format!("{}.envvault", project_name);
    fs::write(&filename, &output)
        .map_err(|e| crate::errors::EnvkeepError::FileWriteError(filename.clone(), e))?;

    println!(
        "{} Exported {} ({} variables) to {}",
        "Done.".green().bold(),
        project_name.cyan(),
        export_data.variables.len(),
        filename.bold()
    );

    Ok(())
}