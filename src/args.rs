use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Args {
    /// KKIIDDZZ.HED path
    pub hed: String,

    /// KKIIDDZZ.DAT path
    pub dat: String,

    /// KKIIDDZZ.BNS path
    pub bns: String,

    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Parser)]
pub struct ExtractArgs {
    /// Index of the file to be extracted. Starts from 0.
    pub index: usize,

    /// Output file
    pub output: String,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// List files inside the KKIIDDZZ.DAT archive
    List,

    /// Extract a file from the KKIIDDZZ.DAT archive
    Extract(ExtractArgs),
}
