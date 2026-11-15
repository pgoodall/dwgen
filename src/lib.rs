use clap::Parser;

/// All options passed are gathered in the Args struct
/// # Example
/// ```
/// use clap::Parser;
/// let args = Args::parse;
/// ```
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Name of local file
    #[arg(short, long)]
    pub file: String,

    /// Number of words
    #[arg(short, long)]
    pub num: Option<u8>,
}