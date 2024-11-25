mod pull;
mod run;
mod solns;

use clap::{Parser, Subcommand};
use std::{fs::File, io::Write};

fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv()?;

    let cli = Cli::parse();

    match cli.command {
        Commands::Run { day } => run::run(day)?,
        Commands::Pull { day } => {
            println!("Pulling for day {}", day);
            let session = std::env::var("SESSION")?;
            let puzzle = pull::pull(&session, day).unwrap();
            let mut file = File::create(format!("./puzzles/day_{:02}.txt", day))?;
            file.write_all(&puzzle)?;
        }
        Commands::Template { day } => {
            let mut file = File::create(format!("./src/solns/day_{:02}.rs", day))?;
            file.write_all(solns::TEMPLATE.as_bytes())?;
            let mut file = File::open("./src/solns/mod.rs")?;
            file.write_all(format!("\npub use day_{:02}", day).as_bytes())?;
        }
    }

    Ok(())
}

#[derive(Parser, Debug)]
#[command(name = "adv")]
#[command(about = "A CLI for Advent of Code", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run the solution for a given day
    Run {
        /// The day of the solution to run
        #[arg(short, long)]
        day: u8,
    },
    /// Pull the puzzle data for the specified day
    Pull {
        /// The day of the puzzle data to pull
        #[arg(short, long)]
        day: u8,
    },
    /// Create a new solution template for the specified day
    Template {
        /// The day to template
        #[arg(short, long)]
        day: u8,
    },
}
