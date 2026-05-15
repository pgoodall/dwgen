use std::io::Error;

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

pub fn process_file(f: &String) -> Result<String, Error> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::open(f)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn clean_word_list(words: Vec<String>) -> Vec<String> {
    words
        .into_iter()
        .filter(|w| w.len() > 3)
        .map(|w| {
            w.replace(
                &['(', ')', ',', '\"', '.', ';', ':', '\'', '!', '-'][..],
                "",
            )
        })
        .collect()
}
