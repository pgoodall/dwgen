//! A command-line utility to generate a diceware-like password
//! consisting of several random words strung together.
use clap::Parser;
use dwgen::{Args, clean_word_list, process_file};
use rand::rngs;

fn main() {
    let args = Args::parse();
    let mut words: Vec<String> = Vec::new();
    if let Ok(contents) = process_file(&args.file) {
        contents
            .split_whitespace()
            .for_each(|s| words.push(s.to_string()));
    }

    // Shuffle the words in the words list
    use rand::seq::{IteratorRandom, SliceRandom};

    let mut rng = rngs::ThreadRng::default();
    let mut word_list: Vec<String> = clean_word_list(words);
    word_list.shuffle(&mut rng);

    // Choose five random words from the list and print them out
    let mut password: Vec<String> = Vec::new();
    for _ in 0..=args.number.unwrap_or(5) {
        if let Some(word) = word_list.iter().choose(&mut rng) {
            password.push(word.to_string().to_lowercase());
        }
    }
    println!("{}", password.join(" "));
}
