//! A command-line utility to generate a diceware-like password
//! consisting of several random words strung together.
use clap::Parser;
use dwgen::Args;

fn main() {
    let args = Args::parse();
    if let Ok(contents) = process_file(&args.file);

}
