use std::env;

fn main() {
    // Example text
    let sample_text = "The quick brown fox jumps over the lazy dog. \
                       This is a simple sentence to demonstrate the algorithm. \
                       Extraordinary complications arise from miscellaneous circumstances. \
                       The complexity of this text should be relatively moderate.";

    let cwd = env::current_dir();
    println!("{cwd:#?}");
}
