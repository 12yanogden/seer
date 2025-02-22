use seek::{init_command, read_pipe, validate_input};
// use std::fs;
// use std::io::{self, Write};

/// The main function that orchestrates the argument parsing, validation, and replacement.
///
/// # Examples
///
/// ```
/// cargo run -- --target "foo" --text "some text"
/// ```
fn main() {
    // Parse arguments
    let matches = init_command().get_matches();
    let pipe = read_pipe();

    let input_validation = validate_input(&matches, &pipe);

    match input_validation {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
