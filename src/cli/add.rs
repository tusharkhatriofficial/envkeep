use anyhow::{Context, Result};
use colored::Colorize;
use std::env;
use std::path::PathBuf;

use crate::crypto;
use crate::env_parser;
use crate::vault;
use crate::vault::project::Project;

pub fn handle_add(name: &str) -> Result<()> {
    // Open the vault
    let password = crypto::prompt_existing_password()?;
    let conn = vault::open_vault(&password)
        .context("Failed to open vault")?;
    let enc_key = vault::get_encryption_key(&conn, &password)?;

    // Find .env in current directory
    let cwd = env::current_dir().context("Could not determine current directory")?;
    let env_path = cwd.join(".env");

    if !env_path.exists() {
        return Err(crate::errors::DotkeepError::NoEnvFile.into());
    }

    // Parse the .env file
    let vars = env_parser::parse_env_file(&env_path)
        .context("Failed to parse .env file")?;

    if vars.is_empty() {
        println!("{} No variables found in .env file.", "Warning:".yellow());
        return Ok(());
    }

    // Create the project
    let directory = cwd.display().to_string();
    let project = Project::new(name, Some(&directory));
    vault::project::create_project(&conn, &project)
        .context("Failed to create project")?;

    // Encrypt and store each variable
    let mut count = 0;
    for (key, value) in &vars {
        let encrypted = crypto::encrypt_value(&enc_key, value)
            .context(format!("Failed to encrypt variable: {}", key))?;
        vault::variable::upsert_variable(&conn, &project.id, key, &encrypted)?;
        count += 1;
    }

    println!(
        "{} Added project {} with {} variables",
        "Done.".green().bold(),
        name.cyan().bold(),
        count
    );
    println!("  Directory: {}", directory);

    Ok(())
}