use clap::ArgMatches;
use seek::{get_file_paths_from_dir, get_searchable, init, read_file, read_pipe, validate_input};

/// The main function that orchestrates the argument parsing, validation, and replacement.
///
/// # Examples
///
/// ```
/// cargo run -- --target "foo" --text "bar"
/// ```
fn main() {
    // Parse arguments
    let matches = init().get_matches();
    let pipe = read_pipe();

    // Validate input
    let input_validation = validate_input(&matches, &pipe);

    match input_validation {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }

    // Search, edit, evaluate, and output
    if let Some(dir) = matches.get_one::<String>("dir") {
        match get_file_paths_from_dir(dir) {
            Ok(file_paths) => {
                for file_path in file_paths {
                    match read_file(file_path.to_str().unwrap()) {
                        Ok(file_content) => run(&matches, file_content),
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
        match get_searchable(&matches, &pipe) {
            Ok(searchable) => run(&matches, searchable),
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }
}

fn run(matches: &ArgMatches, searchable: String) {
    // let search_matches = search(&matches, &searchable);
    // let mut editted_searchable = None;
    // let mut evaluation = None;

    // if let Some(edit_strategy) = get_edit_strategy(&matches) {
    //     editted_searchable = edit(&matches, &searchable, &search_matches, &edit_strategy);
    // }

    // if let Some(evaluation_strategy) = get_evaluation_strategy(&matches) {
    //     evaluation = evaluate(
    //         &matches,
    //         &searchable,
    //         &search_matches,
    //         &editted_searchable,
    //         &evaluation_strategy,
    //     );
    // }

    // output(
    //     &matches,
    //     &searchable,
    //     &search_matches,
    //     &editted_searchable,
    //     &evaluation,
    // );
}
