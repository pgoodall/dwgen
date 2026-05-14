use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Name of local file
    #[arg(short, long)]
    pub file: String,

    /// Number of words
    #[arg(short, long)]
    pub number: Option<u32>,
}

pub fn process_file(f: &String) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let contents = match File::open(f) {
        Ok(f) => f,
        Err(e) => return Err(e),
    }
}