use anyhow::{Context, Result};
use colored::Colorize;
use std::collections::BTreeMap;
use std::env;

use crate::crypto;
use crate::env_parser;
use crate::vault;

pub fn handle_use(project_name: &str) -> Result<()> {
    let password = crypto::prompt_existing_password()?;
    let conn = vault::open_vault(&password)
        .context("Failed to open vault")?;
    let enc_key = vault::get_encryption_key(&conn, &password)?;

    // Get the project
    let project = vault::project::get_project(&conn, project_name)?;

    // Get all variables
    let variables = vault::variable::get_variables(&conn, &project.id)?;

    if variables.is_empty() {
        println!(
            "{} Project {} has no variables.",
            "Warning:".yellow(),
            project_name.cyan()
        );
        return Ok(());
    }

    // Decrypt all values
    let mut vars = BTreeMap::new();
    for var in &variables {
        let decrypted = crypto::decrypt_value(&enc_key, &var.encrypted_value)
            .context(format!("Failed to decrypt variable: {}", var.key))?;
        vars.insert(var.key.clone(), decrypted);
    }

    // Write .env to current directory
    let cwd = env::current_dir().context("Could not determine current directory")?;
    let env_path = cwd.join(".env");

    // Warn if .env already exists
    if env_path.exists() {
        println!(
            "{} Existing .env file will be overwritten.",
            "Warning:".yellow()
        );
    }

    env_parser::write_env_file(&env_path, &vars)?;

    // Update last_used_at
    vault::project::touch_project(&conn, project_name)?;

    println!(
        "{} Wrote {} variables to .env",
        "Done.".green().bold(),
        variables.len()
    );

    Ok(())
}