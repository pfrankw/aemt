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

#[derive(Debug, Parser)]
pub struct PatchArgs {
    /// Index of the file to be extracted. Starts from 0.
    pub index: usize,

    /// Input file to be inserted at the specific index
    pub input: String,
}

#[derive(Debug, Parser)]
pub struct SwapArgs {
    /// Index of the first file to be swapped. Starts from 0.
    pub index_a: usize,

    /// Index of the second file to be swapped. Starts from 0.
    pub index_b: usize,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// List files inside the KKIIDDZZ.DAT archive
    List,

    /// Extract a file from the KKIIDDZZ.DAT archive
    Extract(ExtractArgs),

    /// Patches a file inside the KKIIDDZZ.DAT. The input file is replaced with the one already
    /// present at the same index. The two files must have the same length
    Patch(PatchArgs),

    /// Swap two files inside the KKIIDDZZ.DAT. Not really an useful command for modding. Made just
    /// for testing the archive capabilities.
    Swap(SwapArgs),
}
