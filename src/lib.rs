use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Name of local file
    #[arg(short, long)]
    pub file: String,

    /// Number of words
    #[arg(short, long)]
    pub num: Option<u8>,

    /// Enable interactive mode
    #[arg(short, long)]
    pub interactive: bool
}