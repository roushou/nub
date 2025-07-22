#[derive(thiserror::Error, Debug)]
pub enum CliError {
    #[error("Invalid input")]
    InvalidInput,

    #[error("No languages available")]
    NoLanguagesAvailable,

    #[error("Invalid Language {0}")]
    InvalidLanguage(String),

    #[error("No templates available")]
    NoTemplatesAvailable,

    #[error("Template directory {0} not found")]
    TemplateNotFound(String),

    #[error("Non empty directory {0}")]
    NonEmptyDirectory(String),

    #[error("Malformed go.mod")]
    MalformedGoMod,

    #[error("Invalid file or directory name")]
    InvalidName,

    #[error("Interactive prompt error: {0}")]
    InteractivePrompt(#[from] inquire::InquireError),

    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
}
