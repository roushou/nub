use std::{env, fs, path::Path, str::FromStr};

use clap::ValueEnum;
use include_dir::{Dir, include_dir};

use crate::errors::CliError;

/// Maps hidden files and directories to their correct names
/// Prefixed with "_" instead of "." to allow embedding in the binary.
static RENAME_MAP: &[(&str, &str)] = &[("_github", ".github"), ("_gitignore", ".gitignore")];

#[derive(Clone)]
pub struct Template {
    pub name: String,
    pub language: Language,
}

/// Manages template operations for copying project templates.
#[derive(Clone)]
pub struct TemplateManager {
    templates_dir: Dir<'static>,
}

impl TemplateManager {
    pub fn new() -> Self {
        Self {
            templates_dir: include_dir!("$CARGO_MANIFEST_DIR/../templates"),
        }
    }

    pub fn languages(&self) -> Vec<String> {
        self.templates_dir
            .dirs()
            .filter_map(|dir| dir.path().file_name()?.to_str())
            .map(String::from)
            .collect()
    }

    pub fn templates_for_language(&self, language: Language) -> Vec<Template> {
        let lang_dir = self
            .templates_dir
            .get_dir(language.to_string().to_lowercase());

        lang_dir
            .map(|dir| {
                dir.dirs()
                    .filter_map(|d| {
                        let name = d.path().file_name()?.to_str()?.into();
                        Some(Template { name, language })
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn templates(&self) -> Vec<Template> {
        self.languages()
            .into_iter()
            .flat_map(|lang| {
                let language = lang.parse().unwrap();
                self.templates_for_language(language)
            })
            .collect()
    }

    pub fn copy_template(&self, template_path: &str, destination: &Path) -> Result<(), CliError> {
        let template_dir = self
            .templates_dir
            .get_dir(template_path)
            .ok_or_else(|| CliError::TemplateNotFound(template_path.to_string()))?;

        for entry in template_dir.entries() {
            match entry {
                include_dir::DirEntry::File(file) => {
                    // Copy file directly to destination, ignoring the parent directories
                    let file_name = file
                        .path()
                        .file_name()
                        .ok_or(CliError::InvalidName)?
                        .to_str()
                        .ok_or(CliError::InvalidName)?;
                    let dest_file_name = rename_file(file_name);
                    let dest_path = destination.join(dest_file_name);

                    // Get the current directory name for the module path
                    let current_dir = env::current_dir()?;
                    let dir_name = current_dir
                        .file_name()
                        .ok_or(CliError::InvalidName)?
                        .to_str()
                        .ok_or(CliError::InvalidName)?;

                    let content = if dest_file_name == "go.mod" {
                        let content = std::str::from_utf8(file.contents())
                            .map_err(|_| CliError::InvalidName)?;
                        let updated_content = self.update_go_module_path(content, dir_name)?;
                        updated_content.into_bytes()
                    } else {
                        file.contents().to_vec()
                    };

                    fs::write(&dest_path, content)?;
                }
                include_dir::DirEntry::Dir(dir) => {
                    // Copy subdirectory contents directly to destination
                    let dir_name = dir
                        .path()
                        .file_name()
                        .ok_or(CliError::InvalidName)?
                        .to_str()
                        .ok_or(CliError::InvalidName)?;
                    let dest_dir_name = rename_file(dir_name);
                    let dest_dir = destination.join(dest_dir_name);
                    self.clone().copy_dir_recursive(dir, &dest_dir)?;
                }
            }
        }

        Ok(())
    }

    fn copy_dir_recursive(
        self,
        source: &include_dir::Dir<'_>,
        destination: &Path,
    ) -> Result<(), CliError> {
        if !destination.exists() {
            fs::create_dir_all(destination)?;
        }

        // Copy files
        for file in source.files() {
            let file_name = file.path().file_name().ok_or(CliError::InvalidName)?;
            let file_path = destination.join(file_name);
            fs::write(&file_path, file.contents())?;
        }

        // Recursively copy subdirectories
        for dir in source.dirs() {
            let dir_name = dir.path().file_name().ok_or(CliError::InvalidName)?;
            let dir_path = destination.join(dir_name);
            self.clone().copy_dir_recursive(dir, &dir_path)?;
        }

        Ok(())
    }

    fn update_go_module_path(
        &self,
        content: &str,
        new_module_name: &str,
    ) -> Result<String, CliError> {
        let lines: Vec<&str> = content.lines().collect();
        for (i, line) in lines.iter().enumerate() {
            if line.trim().starts_with("module ") {
                let new_line = format!("module {new_module_name}");
                let mut updated_lines = lines.to_vec();
                updated_lines[i] = &new_line;
                return Ok(updated_lines.join("\n"));
            }
        }
        Err(CliError::MalformedGoMod)
    }
}

fn rename_file(file_name: &str) -> &str {
    RENAME_MAP
        .iter()
        .find(|(src, _)| *src == file_name)
        .map(|(_, dest)| *dest)
        .unwrap_or(file_name)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Language {
    Go,
    Rust,
    Typescript,
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Go => write!(f, "go"),
            Self::Rust => write!(f, "rust"),
            Self::Typescript => write!(f, "typescript"),
        }
    }
}

#[derive(Debug)]
pub struct ParseLanguageError;

impl FromStr for Language {
    type Err = ParseLanguageError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "go" => Ok(Language::Go),
            "golang" => Ok(Language::Go),
            "rust" => Ok(Language::Rust),
            "typescript" => Ok(Language::Typescript),
            _ => Err(ParseLanguageError),
        }
    }
}
