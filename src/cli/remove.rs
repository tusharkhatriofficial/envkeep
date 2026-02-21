use anyhow::{Context, Result};
use colored::Colorize;
use std::io::{self, Write};

use crate::crypto;
use crate::vault;

pub fn handle_remove(name: &str) -> Result<()> {
    let password = crypto::prompt_existing_password()?;
    let conn = vault::open_vault(&password)
        .context("Failed to open vault")?;

    // Check project exists
    let project = vault::project::get_project(&conn, name)?;
    let var_count = vault::project::count_variables(&conn, &project.id)?;

    // Confirm deletion
    print!(
        "Delete project {} ({} variables)? [y/N]: ",
        name.cyan().bold(),
        var_count
    );
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    if input.trim().to_lowercase() != "y" {
        println!("Cancelled.");
        return Ok(());
    }

    vault::project::delete_project(&conn, name)?;

    println!(
        "{} Removed project {} and {} variables",
        "Done.".green().bold(),
        name.cyan(),
        var_count
    );

    Ok(())
}
