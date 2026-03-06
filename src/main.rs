mod env_parser;
mod cli;
mod crypto;
mod errors;
mod tui;
mod vault;


use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands, SecretsAction};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => cli::init::handle_init()?,
        Commands::Add { name } => cli::add::handle_add(&name)?,
        Commands::AddAuto => cli::add_auto::handle_add_auto()?,
        Commands::List => cli::list::handle_list()?,
        Commands::Use { project } => cli::use_project::handle_use(&project)?,
        Commands::Remove { name } => cli::remove::handle_remove(&name)?,
        Commands::Inspect { name } => cli::inspect::handle_inspect(&name)?,
        Commands::Diff { project1, project2 } => cli::diff::handle_diff(&project1, &project2)?,
        Commands::Search { key } => cli::search::handle_search(&key)?,
        Commands::Unused { project } => cli::unused::handle_unused(&project)?,
        Commands::Secrets { action } => cli::secrets::handle_secrets(action)?,
        Commands::Validate { project } => cli::validate::handle_validate(&project)?,
        Commands::Types { project } => cli::types::handle_types(&project)?,
        Commands::Sync { from, to } => cli::sync::handle_sync(&from, &to)?,
        Commands::Generate { template } => cli::generate::handle_generate(&template)?,
        Commands::Export { project } => cli::export::handle_export(&project)?,
        Commands::Import { file } => cli::import::handle_import(&file)?,
        Commands::Backup => cli::backup::handle_backup()?,
        Commands::Restore { file } => cli::restore::handle_restore(&file)?,
        Commands::Status => cli::status::handle_status()?,
        Commands::Recent => cli::recent::handle_recent()?,
        Commands::Migrate => cli::migrate::handle_migrate()?,
        Commands::Tui => println!("envkeep tui: launching..."),
        // _=> {
        //     println!("Command not implemented yet.");
        // }
    }
    Ok(())
}
