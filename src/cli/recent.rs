use anyhow::{Context, Result};
use colored::Colorize;
use std::io::{self, Write};

use crate::crypto;
use crate::vault;

pub fn handle_recent() -> Result<()> {
    let password = crypto::prompt_existing_password()?;
    let conn = vault::open_vault(&password)?;

    let projects = vault::project::list_projects(&conn)?;

    if projects.is_empty() {
        println!("No projects in vault.");
        return Ok(());
    }

    // Show at most 10 recent projects
    let recent: Vec<_> = projects.iter().take(10).collect();

    println!("{}", "Recent projects:".bold());
    println!();

    for (i, project) in recent.iter().enumerate() {
        let var_count = vault::project::count_variables(&conn, &project.id)?;
        let last_used = project
            .last_used_at
            .as_deref()
            .unwrap_or("never");

        println!(
            "  [{}] {} ({} vars, last used: {})",
            i + 1,
            project.name.cyan(),
            var_count,
            last_used.dimmed()
        );
    }

    println!();
    print!("Select project number (or press Enter to cancel): ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();

    if input.is_empty() {
        return Ok(());
    }

    let index: usize = input
        .parse::<usize>()
        .map_err(|_| anyhow::anyhow!("Invalid number"))?;

    if index == 0 || index > recent.len() {
        println!("Invalid selection.");
        return Ok(());
    }

    let selected = &recent[index - 1];
    println!();
    println!("Switching to {}...", selected.name.cyan().bold());

    // Delegate to the use command
    drop(conn); // Close connection before re-opening in handle_use
    crate::cli::use_project::handle_use(&selected.name)?;

    Ok(())
}