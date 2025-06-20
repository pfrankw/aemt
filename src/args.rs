use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Args {
    /// The directory where KKIIDDZZ files are located.
    pub directory: String,

    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Parser)]
pub struct ListArgs {
    /// If set, prints offsets relative to the DAT start instead of the BNS start.
    #[arg(long, default_value = "false")]
    pub true_bns: bool,

    /// If set, prints values in decimal.
    #[arg(long, default_value = "false")]
    pub decimal: bool,
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

    /// Input file to be inserted at the specific index.
    pub input: String,
}

#[derive(Debug, Parser)]
pub struct SwapArgs {
    /// Index of the first file to be swapped. Starts from 0.
    pub index_a: usize,

    /// Index of the second file to be swapped. Starts from 0.
    pub index_b: usize,
}

#[derive(Debug, Parser)]
pub struct HeditArgs {
    /// Index of the file to be modified
    pub index: usize,

    /// New offset
    pub offset: String,

    /// New length
    pub length: String,
}

#[derive(Debug, Parser)]
pub struct PlayArgs {
    /// Index of the file that contains audio tracks
    pub index: usize,

    /// Number of the track to be played
    pub track: Option<usize>,

    /// Shows supported audio configurations
    #[arg(long, default_value = "false")]
    pub show_configs: Option<bool>
}

#[derive(Debug, Parser)]
pub struct ExtractAudioArgs {
    /// Index of the file that contains audio tracks
    pub index: usize,

    /// Number of the track to be extracted
    pub track: usize,

    /// Output file
    pub output: String
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// List files inside the KKIIDDZZ.DAT archive.
    List(ListArgs),

    /// Extract a file from the KKIIDDZZ.DAT archive.
    Extract(ExtractArgs),

    /// Patches a file inside the KKIIDDZZ.DAT. The input file is replaced with the one already
    /// present at the same index. The two files must have the same length.
    Patch(PatchArgs),

    /// Swap two files inside the KKIIDDZZ.DAT. Not really an useful command for modding. Made just
    /// for testing the archive capabilities.
    Swap(SwapArgs),

    /// Raw editing the offset/length pair of each file present in the HED.
    /// Exercise caution as it may break the whole archive.
    /// Generally used for STR files.
    Hedit(HeditArgs),

    /// Decodes ADPCM and plays the sound. Accepts the file index and the track number as
    /// parameters. As of now only works on Linux. Windows build is highly unstable.
    Play(PlayArgs),

    /// Extracts ADPCM from a sound pack inside the archive.
    /// Usually it is the file at index 10 that contains such sounds.
    /// Use vgmstream with a txth file to play them.
    ExtractAudio(ExtractAudioArgs),
}
