use clap::{Parser, Subcommand};
use license::LicenseCommand;
use project::ProjectCommand;

use crate::{commands::template::TemplateCommand, emoji::HUG, errors::CliError};

mod license;
mod project;
mod template;

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help(true))]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

impl Cli {
    pub fn run() -> Result<(), CliError> {
        let cli = Cli::parse();
        println!("{HUG}\n");
        match &cli.command {
            Some(Commands::License(cmd)) => cmd.run(),
            Some(Commands::Project(cmd)) => cmd.run(),
            Some(Commands::Template(cmd)) => cmd.run(),
            None => Ok(()),
        }
    }
}

#[derive(Subcommand)]
enum Commands {
    /// Use a project license
    License(LicenseCommand),

    /// Create a new project in Rust, Go or TypeScript
    Project(ProjectCommand),

    /// Display templates information
    Template(TemplateCommand),
}
