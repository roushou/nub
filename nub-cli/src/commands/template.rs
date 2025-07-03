use clap::{Args, Subcommand};

use crate::{
    errors::CliError,
    template::{Language, TemplateManager},
};

#[derive(Args)]
pub struct TemplateCommand {
    #[command(subcommand)]
    command: SubCommands,
}

impl TemplateCommand {
    pub fn run(&self) -> Result<(), CliError> {
        match &self.command {
            SubCommands::List(cmd) => cmd.run(),
        }
    }
}

#[derive(Subcommand)]
enum SubCommands {
    /// List all templates
    List(ListSubCommand),
}

#[derive(Args)]
struct ListSubCommand {
    #[arg(long, help = "The programming language of the projects to list")]
    language: Option<Language>,
}

impl ListSubCommand {
    pub fn run(&self) -> Result<(), CliError> {
        let manager = TemplateManager::new();
        let templates = manager.templates();

        println!("Available templates\n");
        for template in templates {
            println!("> {} ({})", template.name, template.language);
        }
        Ok(())
    }
}
