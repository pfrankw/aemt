use std::error::Error;

use args::{Args, Command};
use clap::Parser;

mod args;
mod commands;
mod kidz;
mod error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.command {
        Command::List => {commands::list::list(args)?;},
        Command::Extract => {}
    }
    
    Ok(())
}
