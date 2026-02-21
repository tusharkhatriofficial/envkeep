pub mod init;
pub mod add;
pub mod add_auto;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "dotkeep",
    version,
    about = "Keep your .env files safe. Local. Encrypted. Simple.",
    long_about = "dotkeep is a local-first, encrypted CLI + TUI for managing .env \
                  files across all your projects. No cloud. No accounts."
)]

pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    ///create a new vault and set master password
    Init,

    ///upgrade vault schema to latest version
    Migrate,

    ///Export encrypted vault backup
    Backup,

    ///Import vault from backup file
    Restore {
        /// Path to backup file
        file: String,
    },

    ///scan .env file in current directory and store encrypted
    Add {
        /// Project String
        name: String,
    },

    ///Auto detect project name from git remote or directory
    #[command(name = "add-auto")]
    AddAuto,

    ///List all projects present in the vault
    List,

    ///Remove a project from vault
    Remove {
        ///project name to remove
        name: String,
    },

    ///Show project variables (secrets are masked)
    Inspect {
        ///Project name
        name: String,
    },

    ///Compare variable between two projects
    Diff{
        ///First project name
        project1: String,
        ///Second project name
        project2: String,
    },

    ///Write .env file from vault to current directory
    Use{
        project: String,
    },

    ///show the currently active project 
    Status,

    ///Switch to a recently used project
    Recent,

    ///Search for a key across all projects
    Search {
        /// Key name to search for eg DTABASE_URL
        key: String,
    },

    ///find variables no longer referenced in the project code
    Unused{
        ///prohect name
        project: String,
    },

    ///validate variable values for common mistakes
    Validate{
        project: String,
    },

    ///Infer types of variables (string, number, boolean, url etc)
    Types {
        ///Project name
        project: String,
    },

    ///manage shared secrets
    Secrets {
        #[command(subcommand)]
        action: SecretsAction,
    },

    ///Copy common variables from one porject to another
    Sync {
        ///source project 
        from: String,
        ///destination 
        to: String,
    },

    ///Generate .env from a template
    Generate{
        ///template name or its path
        template: String,
    },

    ///Export the project as encrypted .envvault file
    Export {
        ///Project name
        project: String,
    },

    ///Import project from .envvault file
    Import {
        /// path to .envvault file
        file: String,
    },

    ///Launch the full-screen TUI
    Tui,
}

#[derive(Subcommand)]
pub enum SecretsAction{
    ///set as encrypted secret (KEY=VALUE)
    Set{
        ///key= value pair
        pair: String,
    },

    ///List all secrets (values are masked)
    List,

    ///Link a secret to a project
    Link{
        ///Secret key name
        secret: String,
        /// Project name
        project: String,
    },

    ///unlink a secret from a project
    Unlink {
        /// Secret key name
        secret: String,
        ///project name
        project: String,
    },

    ///Rotate a secret (generate new value)
    Rotate {
        ///secret key name
        secret: String,
    },

}