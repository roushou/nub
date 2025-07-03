use std::env;

use clap::{Args, Subcommand};
use inquire::Select;

use crate::{
    errors::CliError,
    template::{Language, Template, TemplateManager},
};

#[derive(Args)]
pub(crate) struct ProjectCommand {
    #[command(subcommand)]
    command: SubCommands,
}

impl ProjectCommand {
    pub fn run(&self) -> Result<(), CliError> {
        match &self.command {
            SubCommands::Create(cmd) => cmd.run(),
        }
    }
}

/// Availalbe subcommands for project operations.
#[derive(Subcommand)]
enum SubCommands {
    /// Create a new project from a template.
    Create(CreateSubCommand),
}

/// Arguments for the create subcommand.
#[derive(Args)]
struct CreateSubCommand {
    #[arg(short, long, help = "The name of the template to use")]
    name: Option<String>,

    #[arg(short, long, help = "The programming language to use")]
    language: Option<Language>,
}

impl CreateSubCommand {
    pub fn run(&self) -> Result<(), CliError> {
        let target_dir = env::current_dir()?;
        self.ensure_empty_directory(&target_dir)?;

        if self.name.is_none() || self.language.is_none() {
            println!("Tell Nub what type of project you want\n");
        }

        let manager = TemplateManager::new();

        let language = self.choose_language(&manager)?;
        let template = self.choose_template(&manager, language)?;
        let template_dir = format!(
            "{}/{}",
            language.to_string().to_lowercase(),
            template.name.to_lowercase(),
        );
        manager.copy_template(&template_dir, &target_dir)?;
        println!("\nProject created!");
        Ok(())
    }

    fn choose_language(&self, manager: &TemplateManager) -> Result<Language, CliError> {
        let languages = manager.languages();
        if languages.is_empty() {
            return Err(CliError::NoLanguagesAvailable);
        }

        let selection = match self.language {
            Some(language) => {
                if !languages.contains(&language.to_string()) {
                    return Err(CliError::InvalidLanguage(language.to_string()));
                }
                println!("> Language: {language}\n");
                language
            }
            None => {
                let selection = Select::new("Select a programming language", languages).prompt()?;
                selection
                    .parse()
                    .map_err(|_| CliError::InvalidLanguage(selection))?
            }
        };
        Ok(selection)
    }

    fn choose_template(
        &self,
        manager: &TemplateManager,
        language: Language,
    ) -> Result<Template, CliError> {
        let templates = manager.templates_for_language(language);
        if templates.is_empty() {
            return Err(CliError::NoTemplatesAvailable);
        }

        let template = match &self.name {
            Some(name) if templates.iter().any(|t| t.name == *name.to_string()) => Some(Template {
                name: name.clone(),
                language,
            }),
            Some(str) => {
                println!("Unknown template {str} for language {language}");
                None
            }
            None => None,
        };

        match template {
            Some(t) => Ok(t),
            None => {
                let names: Vec<_> = templates.iter().map(|t| t.clone().name).collect();
                let selection = Select::new("Select a template", names).prompt()?;
                Ok(Template {
                    name: selection,
                    language,
                })
            }
        }
    }

    fn ensure_empty_directory(&self, dir: &std::path::Path) -> Result<(), CliError> {
        let entries: Vec<_> = std::fs::read_dir(dir)
            .map_err(CliError::Io)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_name() != ".git") // Allow .git/
            .collect();

        if !entries.is_empty() {
            return Err(CliError::NonEmptyDirectory(dir.display().to_string()));
        }
        Ok(())
    }
}
