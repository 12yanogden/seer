use clap::{Arg, ArgGroup, Command};
// use regex::Regex;
// use std::fs;
// use std::io::{self, Read};

/// Parses command line arguments using the `clap` crate.
///
/// # Returns
///
/// A `Command` instance containing the parsed arguments.
///
/// # Examples
///
/// ```
/// use replace::init_command;
/// use clap::CommandFactory;
///
/// let cmd = replace::init_command();
/// let matches = cmd.override_usage("seek --regex <REGEX> --text <TEXT>")
///     .try_get_matches_from(vec!["seek", "--regex", "foo", "--text", "test string"])
///     .unwrap();
/// assert_eq!(matches.get_one::<String>("regex").unwrap(), "foo");
/// assert_eq!(matches.get_one::<String>("text").unwrap(), "test string");
/// ```
pub fn init_command() -> Command {
    Command::new("seek")
        .version("1.0")
        .author("Ryan Ogden <12yanogden@gmail.com>")
        .about("Find/replace matches in a string or file using plain text or regex patterns.")

        // Search strategies group
        .arg(
            Arg::new("target")
                .short('t')
                .long("target")
                .value_name("TARGET")
                .help("A plain text string to find")
                .action(clap::ArgAction::Set),
        )
        .arg(
            Arg::new("regex")
                .short('x')
                .long("regex")
                .value_name("REGEX")
                .help("A regex pattern to match")
                .action(clap::ArgAction::Set),
        )
        .arg(
            Arg::new("between")
                .short('b')
                .long("between")
                .help("Two regex patterns to search for. Find/replace both matches and the text between. Use --exclude_matches to only find/replace the text between.")
                .num_args(2)
                .value_names(&["begin", "end"]),
        )
        .group(
            ArgGroup::new("search_strategies")
                .args(&["target", "regex", "between"])
                .required(true)
                .multiple(false),
        )

        // Between group
        .arg(
            Arg::new("exclude_matches")
                .short('e')
                .long("exclude_matches")
                .help("Only find/replace the text between the two regex patterns")
                .action(clap::ArgAction::SetTrue),
        )
        .group(
            ArgGroup::new("between_group")
                .args(["exclude_matches"])
                .requires("between"),
        )

        // Requires group
        .arg(
            Arg::new("only_required_for_required_group")
                .long("only_required_for_required_group")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("required_for_dependent_and_required_group")
                .long("required_for_dependent_and_required_group")
                .action(clap::ArgAction::SetTrue)
        )
        .group(
            ArgGroup::new("required_group")
                .args(["only_required_for_required_group", "required_for_dependent_and_required_group"])
                .required(true),
        )
        .arg(
            Arg::new("dependent")
                .long("dependent")
                .action(clap::ArgAction::SetTrue)
        )
        .group(
            ArgGroup::new("requires_group")
                .args(["dependent"])
                .requires("required_for_dependent_and_required_group")
        )

        // Edit stragegies group
        .arg(
            Arg::new("prepend")
                .short('p')
                .long("prepend")
                .value_name("PREPEND")
                .help("Prepend the string to matches")
                .action(clap::ArgAction::Set),
        )
        .arg(
            Arg::new("replace_with")
                .short('r')
                .long("replace_with")
                .value_name("REPLACE_WITH")
                .help("The string to replace matches with")
                .action(clap::ArgAction::Set),
        )
        .arg(
            Arg::new("append")
                .short('a')
                .long("append")
                .value_name("APPEND")
                .help("Append the string to matches")
                .action(clap::ArgAction::Set),
        )
        .group(
            ArgGroup::new("edit_startegies")
                .args(["prepend", "replace_with", "append"])
                .required(false)
                .multiple(false),
        )
        

        // Searchable group
        .arg(
            Arg::new("text")
                .long("text")
                .value_name("TEXT")
                .help("The string to search within")
                .action(clap::ArgAction::Set),
        )
        .arg(
            Arg::new("file")
                .long("file")
                .value_name("FILE")
                .help("The file to search within")
                .action(clap::ArgAction::Set),
        )
        .group(
            ArgGroup::new("searchable")
                .args(["text", "file"])
                .required(true)
                .multiple(false),
        )

        // File group
        .arg(
            Arg::new("in_place")
                .short('i')
                .long("in_place")
                .help("Edit the file in place")
                .action(clap::ArgAction::SetTrue),
        )
        .group(
            ArgGroup::new("file_group")
                .args(["file", "in_place"])
                .required(false)
                .requires("file"),
        )

        // Frequency group
        .arg(
            Arg::new("nth")
                .long("nth")
                .value_name("NTH")
                .help("Find/replace only the nth match")
                .value_parser(clap::value_parser!(u16).range(0..))
                .action(clap::ArgAction::Set),
        )
        .arg(
            Arg::new("every_nth")
                .long("every_nth")
                .value_name("EVERY_NTH")
                .help("Find/replace every nth match")
                .value_parser(clap::value_parser!(u16).range(0..))
                .action(clap::ArgAction::Set),
        )
        .arg(
            Arg::new("all")
                .long("all")
                .help("Find/replace all matches")
                .action(clap::ArgAction::SetTrue),
        )
        .group(
            ArgGroup::new("frequency")
                .args(["nth", "every_nth", "all"])
                .required(false)
                .multiple(false),
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_cmd::assert;
    use clap::error::ErrorKind;

    lazy_static::lazy_static! {
        static ref CMD: Command = init_command();
    }

    #[test]
    fn test_one_and_only_one_search_strategy_can_be_given() {
        // Test with only --target
        let matches = CMD.clone().try_get_matches_from(vec![
            "seek",
            "--target", "foo", 
            "--text", "bar"
        ]);
        assert!(matches.is_ok());

        // Test with only --regex
        let matches = CMD.clone().try_get_matches_from(vec![
            "seek",
            "--regex", "foo",
            "--text", "bar"
        ]);
        assert!(matches.is_ok());

        // Test with only --between
        let matches = CMD.clone().try_get_matches_from(vec![
            "seek",
            "--between", "foo", "bar",
            "--text", "bar"
        ]);
        assert!(matches.is_ok());

        // Test with --target and --regex (should fail)
        let matches = CMD.clone().try_get_matches_from(vec![
            "seek",
            "--target", "foo",
            "--regex", "bar",
            "--text", "bar"
        ]);
        assert!(matches.is_err());
        let err = matches.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::ArgumentConflict);

        // Test with --target and --between (should fail)
        let matches = CMD.clone().try_get_matches_from(vec![
            "seek",
            "--target", "foo",
            "--between", "foo", "bar",
            "--text", "bar"
        ]);
        assert!(matches.is_err());
        let err = matches.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::ArgumentConflict);

        // Test with --regex and --between (should fail)
        let matches = CMD.clone().try_get_matches_from(vec![
            "seek",
            "--regex", "foo",
            "--between", "foo", "bar",
            "--text", "bar"
        ]);
        assert!(matches.is_err());
        let err = matches.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::ArgumentConflict);
    }

    #[test]
    fn test_exclusive_flag_requires_between_flag() {
        // Test with only --exclusive (should fail)
        let matches = CMD.clone().try_get_matches_from(vec![
            "seek",
            "--target", "foo",
            "--text", "bar",
            "--exclude_matches"
        ]);
        assert!(matches.is_err());
        let err = matches.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);

        // Test with --between and --exclusive (should pass)
        let matches = CMD.clone().try_get_matches_from(vec![
            "seek",
            "--between", "foo", "bar",
            "--text", "bar",
            "--exclude_matches"
        ]);
        assert!(matches.is_ok());
    }

    #[test]
    fn test_requires_flag_from_different_required_group() {
        // Test with only --color (should fail)
        let matches = CMD.clone().try_get_matches_from(vec![
            "seek",
            "--only_required_for_required_group",
            "--dependent",
            "--target", "foo",
            "--text", "bar"
        ]);
        assert!(matches.is_err());
        let err = matches.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);
    }
}

// /// Validates that the given pattern is a valid regular expression.
// ///
// /// # Arguments
// ///
// /// * `pattern` - A string slice that holds the regex pattern.
// ///
// /// # Examples
// ///
// /// ```
// /// use replace::verify_is_valid_regex;
// /// verify_is_valid_regex(r"\d+");
// /// ```
// pub fn verify_is_valid_regex(pattern: &str) {
//     if Regex::new(pattern).is_err() {
//         eprintln!(
//             "Error: The pattern given is not a valid regular expression: {}",
//             pattern
//         );
//         std::process::exit(1);
//     }
// }

// /// Validates that at least one of the conflicting options in each pair is undefined.
// ///
// /// # Arguments
// ///
// /// * `option_pairs` - A list of tuples containing pairs of conflicting option values.
// ///
// /// # Examples
// ///
// /// ```
// /// use replace::verify_has_no_conflicting_options;
// /// verify_has_no_conflicting_options(vec![(Some("value1"), None), (None, Some("value2"))]);
// /// ```
// pub fn verify_has_no_conflicting_options(option_pairs: Vec<(Option<&str>, Option<&str>)>) {
//     for (opt1, opt2) in option_pairs {
//         if opt1.is_some() && opt2.is_some() {
//             eprintln!(
//                 "error: conflicting options provided: {:?}, {:?}",
//                 opt1.unwrap(),
//                 opt2.unwrap()
//             );
//             std::process::exit(1);
//         }
//     }
// }

// /// Verifies that at least one option is provided.
// ///
// /// # Arguments
// ///
// /// * `option_groups` - A list of vectors, each containing options.
// ///
// /// # Panics
// ///
// /// This function will panic if none of the options are provided.
// ///
// /// # Examples
// ///
// /// ```
// /// use replace::verify_at_least_one_option_is_provided;
// ///
// /// // This will not panic
// /// verify_at_least_one_option_is_provided(vec![
// ///     vec![Some("value1"), None],
// ///     vec![None, Some("value2")],
// /// ]);
// ///
// /// // This will panic
// /// // verify_at_least_one_option_is_provided(vec![
// /// //     vec![None, None],
// /// //     vec![None, None],
// /// // ]);
// /// ```
// pub fn verify_at_least_one_option_is_provided(option_groups: Vec<Vec<Option<&str>>>) {
//     let at_least_one_provided = option_groups
//         .iter()
//         .any(|group| group.iter().any(|opt| opt.is_some()));

//     if !at_least_one_provided {
//         let option_names: Vec<&str> = option_groups
//             .iter()
//             .flat_map(|group| group.iter().map(|opt| opt.unwrap_or("None")))
//             .collect();
//         panic!(
//             "error: at least one option must be provided: {}",
//             option_names.join(", ")
//         );
//     }
// }

// /// Verifies that the given file exists.
// ///
// /// # Arguments
// ///
// /// * `file_path` - A string slice that holds the file path.
// ///
// /// # Panics
// ///
// /// This function will panic if the file does not exist.
// ///
// /// # Examples
// ///
// /// ```
// /// use tempfile::NamedTempFile;
// /// use replace::verify_file_exists;
// ///
// /// let file = NamedTempFile::new().unwrap();
// /// verify_file_exists(file.path().to_str().unwrap());
// /// ```
// pub fn verify_file_exists(file_path: &str) {
//     if !fs::metadata(file_path).is_ok() {
//         eprintln!("error: the file does not exist: {}", file_path);
//         std::process::exit(1);
//     }
// }

// /// Finds all matches of the given pattern in the content string.
// ///
// /// # Arguments
// ///
// /// * `pattern` - A regex pattern to match.
// /// * `content` - The string to search within.
// ///
// /// # Returns
// ///
// /// A vector of tuples where each tuple contains the start and end indices of a match.
// ///
// /// # Examples
// ///
// /// ```
// /// use replace::find_matches;
// /// let matches = find_matches(r"\d+", "123 abc 456");
// /// assert_eq!(matches, vec![(0, 3), (8, 11)]);
// /// ```
// pub fn find_matches(pattern: &str, content: &str) -> Vec<(usize, usize)> {
//     let re = Regex::new(pattern).expect(&format!("Invalid regex pattern: {}", pattern));
//     re.find_iter(content)
//         .map(|found_match| (found_match.start(), found_match.end()))
//         .collect()
// }

// /// Reads piped input from stdin.
// ///
// /// # Returns
// ///
// /// An `Option<String>` containing the piped input if available.
// ///
// /// # Examples
// ///
// /// ```
// /// use replace::read_pipe;
// ///
// /// if let Some(pipe_input) = read_pipe() {
// ///     println!("Piped input: {}", pipe_input);
// /// }
// /// ```
// pub fn read_pipe() -> Option<String> {
//     let mut pipe = String::new();
//     let could_read_input = io::stdin().read_to_string(&mut pipe).is_ok();

//     if could_read_input && !pipe.is_empty() {
//         Some(pipe)
//     } else {
//         None
//     }
// }
