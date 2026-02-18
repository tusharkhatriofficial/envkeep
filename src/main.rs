mod cli;
mod crypto;
mod vault;
mod tui;
mod errors;

use clap::Parser;
use cli::{Cli, Commands, SecretsAction};
use anyhow::Result;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => println!("dotkeep init: creating vault..."),
        Commands::Migrate => println!("dotkeep migrate: upgrading schema..."),
        Commands::Backup => println!("dotkeep backup: exporting vault..."),
        Commands::Restore { file } => println!("dotkeep restore: importing from {}...", file),
        Commands::Add { name } => println!("dotkeep add: scanning .env for {}...", name),
        Commands::AddAuto => println!("dotkeep add-auto: detecting project..."),
        Commands::List => println!("dotkeep list: showing all projects..."),
        Commands::Remove { name } => println!("dotkeep remove: deleting {}...", name),
        Commands::Inspect { name } => println!("dotkeep inspect: showing {}...", name),
        Commands::Diff { project1, project2 } => {
            println!("dotkeep diff: comparing {} vs {}...", project1, project2);
        }
        Commands::Use { project } => println!("dotkeep use: writing .env for {}...", project),
        Commands::Status => println!("dotkeep status: showing active project..."),
        Commands::Recent => println!("dotkeep recent: showing recent projects..."),
        Commands::Search { key } => println!("dotkeep search: finding {}...", key),
        Commands::Unused { project } => println!("dotkeep unused: checking {}...", project),
        Commands::Validate { project } => println!("dotkeep validate: checking {}...", project),
        Commands::Types { project } => println!("dotkeep types: inferring for {}...", project),
        Commands::Secrets { action } => match action {
            SecretsAction::Set { pair } => println!("dotkeep secrets set: {}...", pair),
            SecretsAction::List => println!("dotkeep secrets list..."),
            SecretsAction::Link { secret, project } => {
                println!("dotkeep secrets link: {} -> {}...", secret, project);
            }
            SecretsAction::Unlink { secret, project } => {
                println!("dotkeep secrets unlink: {} -> {}...", secret, project);
            }
            SecretsAction::Rotate { secret } => {
                println!("dotkeep secrets rotate: {}...", secret);
            }
        },
        Commands::Sync { from, to } => println!("dotkeep sync: {} -> {}...", from, to),
        Commands::Generate { template } => println!("dotkeep generate: from {}...", template),
        Commands::Export { project } => println!("dotkeep export: {}...", project),
        Commands::Import { file } => println!("dotkeep import: from {}...", file),
        Commands::Tui => println!("dotkeep tui: launching..."),
        // _=> {
        //     println!("Command not implemented yet.");
        // }
    }
    Ok(())
}
