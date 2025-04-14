use clap::ArgMatches;
use parse::search::search;
use parse::{get_file_paths_from_dir, get_searchable, init, read_file, read_pipe, validate_input}; // Add this line to import the search function

/// The main function that orchestrates the argument parsing, validation, and replacement.
///
/// # Examples
///
/// ```
/// cargo run -- --exact "foo" --text "bar"
/// ```
fn main() {
    let inputs = CommandBuilder.build().get_matches();
    let pipe = read_pipe();

    // Validate input
    let input_validation = validate_input(&inputs, &pipe);

    match input_validation {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }

    // Search, edit, evaluate, and output
    if let Some(dir) = inputs.get_one::<String>("dir") {
        match get_file_paths_from_dir(dir) {
            Ok(file_paths) => {
                for file_path in file_paths {
                    match read_file(file_path.to_str().unwrap()) {
                        Ok(file_content) => run(&inputs, file_content),
                        Err(e) => {
                            eprintln!("{}", e);
                            std::process::exit(1);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    } else {
        match get_searchable(&inputs, &pipe) {
            Ok(searchable) => run(&inputs, searchable),
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }
}

fn run(inputs: &ArgMatches, searchable: String) {
    let search_matches = search(inputs, searchable); // Call the search function here
                                                     // let mut editted_searchable = None;
                                                     // let mut evaluation = None;

    // if let Some(edit_strategy) = get_edit_strategy(&inputs) {
    //     editted_searchable = edit(&inputs, &searchable, &search_matches, &edit_strategy);
    // }

    // if let Some(evaluation_strategy) = get_evaluation_strategy(&inputs) {
    //     evaluation = evaluate(
    //         &inputs,
    //         &searchable,
    //         &search_matches,
    //         &editted_searchable,
    //         &evaluation_strategy,
    //     );
    // }

    // output(
    //     &inputs,
    //     &searchable,
    //     &search_matches,
    //     &editted_searchable,
    //     &evaluation,
    // );
}
