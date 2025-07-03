mod cli;
mod errors;

use cli::Cli;

fn main() {
    if let Err(err) = Cli::run() {
        eprintln!("CLI encountered an error: {err}")
    }
}
