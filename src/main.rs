use std::error::Error;

use args::{Args, Command};
use clap::Parser;

mod args;
mod commands;
mod error;
mod kidz;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match &args.command {
        Command::List => {
            commands::list::list(&args)?;
        }
        Command::Extract(eargs) => {
            commands::extract::extract(&args, eargs)?;
        }
        Command::Patch(eargs) => {
            commands::patch::patch(&args, eargs)?;
        }
        Command::Swap(eargs) => {
            commands::swap::swap(&args, eargs)?;
        }

    }

    Ok(())
}
