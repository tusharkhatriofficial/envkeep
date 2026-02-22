use anyhow::{Context, Result};
use colored::Colorize;

use crate::crypto;
use crate::vault;

pub fn handle_init() -> Result<()> {
    // Check if vault already exists
    if vault::vault_exists() {
        let path = vault::vault_path();
        println!(
            "{} Vault already exists at {}",
            "Error:".red().bold(),
            path.display()
        );
        println!("  Run {} to open the existing vault.", "envkeep list".cyan());
        return Ok(());
    }

    println!("{}", "envkeep -- Keep your .env files safe".bold());
    println!();
    println!("Creating a new vault. Choose a master password.");
    println!("This password encrypts all your secrets. {}", "Do not forget it.".yellow().bold());
    println!();

    // Prompt for master password
    let password = crypto::prompt_new_password()
        .context("Failed to read password")?;

    // Create the vault
    vault::create_vault(&password)
        .context("Failed to create vault")?;

    let path = vault::vault_path();
    println!();
    println!("{} Vault created at {}", "Done.".green().bold(), path.display());
    println!();
    println!("Next steps:");
    println!("  1. cd into a project directory");
    println!("  2. Run {} to store its .env file", "envkeep add <name>".cyan());

    Ok(())
}