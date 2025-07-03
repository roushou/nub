use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::errors::CliError;

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help(true))]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

impl Cli {
    pub fn run() -> Result<(), CliError> {
        let cli = Cli::parse();
        match &cli.command {
            Some(Commands::Fruit(cmd)) => cmd.run(),
            None => Err(CliError::MissingCommand),
        }
    }
}

#[derive(Subcommand)]
enum Commands {
    /// Eat 5 fruits everyday
    Fruit(FruitCommand),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Fruit {
    Apple,
    Banana,
    Orange,
    Strawberry,
    Watermelon,
}

impl std::fmt::Display for Fruit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Apple => write!(f, "apple"),
            Self::Banana => write!(f, "banana"),
            Self::Orange => write!(f, "orange"),
            Self::Strawberry => write!(f, "strawberry"),
            Self::Watermelon => write!(f, "watermelon"),
        }
    }
}

#[derive(Args)]
pub struct FruitCommand {
    #[arg(short, long, help = "List all fruits")]
    list: bool,
}

impl FruitCommand {
    pub fn run(&self) -> Result<(), CliError> {
        if self.list {
            println!("Apple, Banana, Orange, Strawberry, Watermelon")
        } else {
            println!("Don't forget to eat your fruits!")
        }
        Ok(())
    }
}
