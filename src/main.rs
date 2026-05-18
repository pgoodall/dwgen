//! A command-line utility to generate a diceware-like password
//! consisting of several random words strung together.
use clap::Parser;
use dwgen::{Args, generate_password};

fn main() {
    let args = Args::parse();
    let password = generate_password(args);

    println!("{}", password.join(" "));
}
