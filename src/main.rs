//! A command-line utility to generate a diceware-like password
//! consisting of several random words strung together.
use clap::Parser;
use dwgen_lib::{Config, Source, generate_password};
use dwgen::interactive;
use dwgen::Cli;

fn main() {
    let args = Cli::parse();
    let mut config = Config {
        source: Source::File("example.txt".to_string()),
        num: None,
    };

    if args.interactive {
        if let Ok(path) = interactive::prompt_filename() {
            config.source = Source::File(path);
        }
    }
    
    if let Some(path) = args.file {
        config = Config {
            source: Source::File(path),
            num: args.num,
        };
    }

    let password = generate_password(config);

    println!("{}", password.join(" "));
}
