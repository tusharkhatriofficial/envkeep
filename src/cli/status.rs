use anyhow::{Context, Result};
use colored::Colorize;

use crate::crypto;
use crate::vault;

pub fn handle_status() -> Result<()> {
    let password = crypto::prompt_existing_password()?;
    let conn = vault::open_vault(&password)?;

    // Find the most recently used project
    let projects = vault::project::list_projects(&conn)?;

    if projects.is_empty() {
        println!("No projects in vault.");
        return Ok(());
    }

    // The list is ordered by last_used_at DESC
    let active = &projects[0];

    // Check if the current directory matches any project
    let cwd = std::env::current_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_default();

    let current_project = projects
        .iter()
        .find(|p| p.directory.as_deref() == Some(&cwd));

    println!("{}", "envkeep status".bold());
    println!();

    if let Some(project) = current_project {
        let var_count = vault::project::count_variables(&conn, &project.id)?;
        println!(
            "  Current directory matches: {} ({} vars)",
            project.name.cyan().bold(),
            var_count
        );
    } else {
        println!("  Current directory: {} (no matching project)", cwd.dimmed());
    }

    println!();
    println!(
        "  Last used project: {} ({})",
        active.name.cyan(),
        active
            .last_used_at
            .as_deref()
            .unwrap_or("never")
    );

    println!("  Total projects: {}", projects.len());

    Ok(())
}