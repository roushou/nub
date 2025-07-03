#[derive(thiserror::Error, Debug)]
pub enum CliError {
    #[error("Missing command")]
    MissingCommand,
}
