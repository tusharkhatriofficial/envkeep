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

        // placeholder commands for now - will implement later
        Commands::Migrate => println!("envkeep migrate: upgrading schema..."),
        Commands::Backup => println!("envkeep backup: exporting vault..."),
        Commands::Restore { file } => println!("envkeep restore: importing from {}...", file),
        // Commands::Add { name } => println!("envkeep add: scanning .env for {}...", name),
        // Commands::AddAuto => println!("envkeep add-auto: detecting project..."),
        // Commands::List => println!("envkeep list: showing all projects..."),
        // Commands::Remove { name } => println!("envkeep remove: deleting {}...", name),
        Commands::Inspect { name } => println!("envkeep inspect: showing {}...", name),
        Commands::Diff { project1, project2 } => {
            println!("envkeep diff: comparing {} vs {}...", project1, project2);
        }
        // Commands::Use { project } => println!("envkeep use: writing .env for {}...", project),
        Commands::Status => println!("envkeep status: showing active project..."),
        Commands::Recent => println!("envkeep recent: showing recent projects..."),
        Commands::Search { key } => println!("envkeep search: finding {}...", key),
        Commands::Unused { project } => println!("envkeep unused: checking {}...", project),
        Commands::Validate { project } => println!("envkeep validate: checking {}...", project),
        Commands::Types { project } => println!("envkeep types: inferring for {}...", project),
        Commands::Secrets { action } => match action {
            SecretsAction::Set { pair } => println!("envkeep secrets set: {}...", pair),
            SecretsAction::List => println!("envkeep secrets list..."),
            SecretsAction::Link { secret, project } => {
                println!("envkeep secrets link: {} -> {}...", secret, project);
            }
            SecretsAction::Unlink { secret, project } => {
                println!("envkeep secrets unlink: {} -> {}...", secret, project);
            }
            SecretsAction::Rotate { secret } => {
                println!("envkeep secrets rotate: {}...", secret);
            }
        },
        Commands::Sync { from, to } => println!("envkeep sync: {} -> {}...", from, to),
        Commands::Generate { template } => println!("envkeep generate: from {}...", template),
        Commands::Export { project } => println!("envkeep export: {}...", project),
        Commands::Import { file } => println!("envkeep import: from {}...", file),
        Commands::Tui => println!("envkeep tui: launching..."),
        // _=> {
        //     println!("Command not implemented yet.");
        // }
    }
    Ok(())
}
