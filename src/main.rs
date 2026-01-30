fn read_cli_args() -> Vec<String> {
    use std::env;
    let args: Vec<String> = env::args().collect();
    return args
}


fn main() {
    let args: Vec<String> = read_cli_args();
    for arg in &args {
        println!("{arg}");
    }
}
