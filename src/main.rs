//! A command-line utility to generate a diceware-like password
//! consisting of several random words strung together.
use clap::Parser;
use dwgen_lib::{Config, generate_password};
use dwgen::Cli;

fn main() {
    let args = Cli::parse();
    let config = Config {
        file: args.file,
        num: args.num,
    };
    let password = generate_password(config);

    println!("{}", password.join(" "));
}
