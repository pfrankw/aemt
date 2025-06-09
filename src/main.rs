use std::error::Error;

use args::{Args, Command};
use clap::Parser;

mod args;
mod commands;
mod error;
mod kidz;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match &args.command {
        Command::List(eargs) => {
            commands::list::list(&args, eargs)?;
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
        Command::Hedit(eargs) => {
            commands::hedit::hedit(&args, eargs)?;
        }
        Command::Play(eargs) => {
            commands::play::play(&args, eargs)?;
        }
        Command::ExtractAudio(eargs) => {
            commands::extract_audio::extract_audio(&args, eargs)?;
        }
    }

    Ok(())
}
