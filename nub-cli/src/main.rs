mod commands;
mod emoji;
mod errors;
mod template;

use commands::Cli;
use emoji::{CRY_WAVE, DEAL_WITH_IT, EXCITED, FLIP_TABLE};
use errors::CliError;
use inquire::InquireError;

fn main() {
    match Cli::run() {
        Ok(_) => println!("\n{EXCITED}"),
        Err(CliError::InteractivePrompt(err)) => {
            match err {
                InquireError::OperationCanceled => println!("\n{FLIP_TABLE}"),
                InquireError::OperationInterrupted => println!("\n{FLIP_TABLE}"),
                _ => println!("Nub encountered an error {err}"),
            }
            std::process::exit(1);
        }
        Err(CliError::NonEmptyDirectory(_)) => {
            eprintln!("{CRY_WAVE}\n\nNub needs an empty directory");
        }
        Err(err) => {
            eprintln!("{DEAL_WITH_IT}\n\nOops, something unexpected happened: {err}");
        }
    }
}
