//! A command-line utility to generate a diceware-like password
//! consisting of several random words strung together.
use clap::Parser;
use dwgen_lib::{Config, generate_password};
use dwgen::Cli;

/// The only required parameter is `--file`. Optinally add 
/// `--num` to specify the number of words returned.
/// # Example
/// ```
/// dwgen --file tests/example.txt
/// ```

fn main() {
    let args = Cli::parse();
    let config = Config {
        file: args.file,
        num: args.num,
    };
    let password = generate_password(config);

    println!("{}", password.join(" "));
}
