use cliclack::input;

pub fn prompt_filename() -> Result<String, std::io::Error> {
    let path: String = input("Enter the path to a text file with lots of words")
    .placeholder("./test/example-text.txt")
    .validate(|input: &String| {
        if input.is_empty() {
            Err("Please enter a file name.")
        } else {
            Ok(())
        }
    })
    .interact()?;

    Ok(path)
}
