use anyhow::{Context, Result};
use colored::Colorize;
use std::fs;
use std::path::Path;

use crate::crypto;
use crate::vault;

pub fn handle_unused(project_name: &str) -> Result<()> {
    let password = crypto::prompt_existing_password()?;
    let conn = vault::open_vault(&password)
        .context("Failed to open vault")?;

    let project = vault::project::get_project(&conn, project_name)?;
    let variables = vault::variable::get_variables(&conn, &project.id)?;

    if variables.is_empty() {
        println!("Project {} has no variables.", project_name.cyan());
        return Ok(());
    }

    // Determine the search directory
    let search_dir = match &project.directory {
        Some(dir) => dir.clone(),
        None => {
            println!(
                "{} No directory stored for this project. Using current directory.",
                "Warning:".yellow()
            );
            std::env::current_dir()?.display().to_string()
        }
    };

    let search_path = Path::new(&search_dir);
    if !search_path.exists() {
        println!(
            "{} Directory {} does not exist.",
            "Error:".red(),
            search_dir
        );
        return Ok(());
    }

    println!(
        "Checking {} variables in {}...",
        variables.len(),
        search_dir.dimmed()
    );

    // Collect all source file contents
    let source_content = collect_source_files(search_path)?;

    let mut unused_keys = Vec::new();
    let mut used_keys = Vec::new();

    for var in &variables {
        // Search for the key name in source files
        let is_used = source_content
            .iter()
            .any(|(_, content)| content.contains(&var.key));

        if is_used {
            used_keys.push(var.key.clone());
        } else {
            unused_keys.push(var.key.clone());
        }
    }

    if unused_keys.is_empty() {
        println!(
            "{} All {} variables are referenced in source code.",
            "Done.".green().bold(),
            used_keys.len()
        );
    } else {
        println!();
        println!(
            "{} {} potentially unused variables:",
            "Found".yellow().bold(),
            unused_keys.len()
        );
        for key in &unused_keys {
            println!("  {} {}", "?".yellow(), key);
        }
        println!();
        println!(
            "  {} used, {} potentially unused out of {} total",
            used_keys.len(),
            unused_keys.len(),
            variables.len()
        );
        println!();
        println!(
            "{}",
            "Note: check manually -- some vars may be used dynamically.".dimmed()
        );
    }

    Ok(())
}

/// Recursively collect source file contents from a directory.
/// Skips binary files, node_modules, .git, target, etc.
fn collect_source_files(dir: &Path) -> Result<Vec<(String, String)>, anyhow::Error> {
    let mut files = Vec::new();
    collect_files_recursive(dir, &mut files)?;
    Ok(files)
}

fn collect_files_recursive(
    dir: &Path,
    files: &mut Vec<(String, String)>,
) -> Result<(), anyhow::Error> {
    // Skip common directories
    let skip_dirs = [
        "node_modules", ".git", "target", "dist", "build",
        ".next", "__pycache__", "venv", ".venv",
    ];

    let dir_name = dir
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();

    if skip_dirs.contains(&dir_name.as_str()) {
        return Ok(());
    }

    if !dir.is_dir() {
        return Ok(());
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            collect_files_recursive(&path, files)?;
        } else if path.is_file() {
            // Only read text-like source files
            let ext = path
                .extension()
                .map(|e| e.to_string_lossy().to_string())
                .unwrap_or_default();

            let source_extensions = [
                "rs", "js", "ts", "jsx", "tsx", "py", "rb", "go", "java",
                "kt", "swift", "c", "cpp", "h", "cs", "php", "yaml", "yml",
                "toml", "json", "xml", "html", "css", "scss", "sh", "bash",
                "zsh", "fish", "dockerfile", "makefile", "md",
            ];

            if source_extensions.contains(&ext.to_lowercase().as_str()) {
                if let Ok(content) = fs::read_to_string(&path) {
                    files.push((path.display().to_string(), content));
                }
            }
        }
    }

    Ok(())
}