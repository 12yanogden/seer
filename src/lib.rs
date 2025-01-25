use clap::{Arg, Command};
use regex::Regex;

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
/// let matches = cmd.override_usage("replace --pattern <PATTERN> --replacement <REPLACEMENT> --haystack <HAYSTACK>")
///     .try_get_matches_from(vec!["replace", "--pattern", "foo", "--replacement", "bar", "--haystack", "test string"])
///     .unwrap();
/// assert_eq!(matches.get_one::<String>("pattern").unwrap(), "foo");
/// assert_eq!(matches.get_one::<String>("replacement").unwrap(), "bar");
/// assert_eq!(matches.get_one::<String>("haystack").unwrap(), "test string");
/// ```
pub fn init_command() -> Command {
    Command::new("replace")
        .version("1.0")
        .author("Ryan Ogden <12yanogden@gmail.com>")
        .about("Replaces matches to a regex in a string with a given string")
        .arg(
            Arg::new("pattern")
                .short('p')
                .long("pattern")
                .value_name("PATTERN")
                .help("The regex pattern to match")
                .action(clap::ArgAction::Set)
                .required(true),
        )
        .arg(
            Arg::new("replacement")
                .short('r')
                .long("replacement")
                .value_name("REPLACEMENT")
                .help("The string to replace matches with")
                .action(clap::ArgAction::Set)
                .required(true),
        )
        .arg(
            Arg::new("haystack")
                .long("haystack")
                .value_name("HAYSTACK")
                .help("The string to search within")
                .action(clap::ArgAction::Set)
                .required(true),
        )
        .arg(
            Arg::new("all")
                .long("all")
                .help("Replace all matches of the pattern")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("every_nth")
                .long("every_nth")
                .value_name("EVERY_NTH")
                .help("Replace every nth match of the pattern")
                .value_parser(clap::value_parser!(u16).range(0..))
                .action(clap::ArgAction::Set),
        )
        .arg(
            Arg::new("nth")
                .long("nth")
                .value_name("NTH")
                .help("Replace only the nth match of the pattern")
                .value_parser(clap::value_parser!(u16).range(0..))
                .action(clap::ArgAction::Set),
        )
}

/// Validates that the given pattern is a valid regular expression.
///
/// # Arguments
///
/// * `pattern` - A string slice that holds the regex pattern.
///
/// # Examples
///
/// ```
/// use replace::verify_is_valid_regex;
/// verify_is_valid_regex(r"\d+");
/// ```
pub fn verify_is_valid_regex(pattern: &str) {
    if Regex::new(pattern).is_err() {
        eprintln!(
            "Error: The pattern given is not a valid regular expression: {}",
            pattern
        );
        std::process::exit(1);
    }
}

/// Validates that at least one of the conflicting options in each pair is undefined.
///
/// # Arguments
///
/// * `option_pairs` - A list of tuples containing pairs of conflicting option values.
///
/// # Examples
///
/// ```
/// use replace::verify_has_no_conflicting_options;
/// verify_has_no_conflicting_options(vec![(Some("value1"), None), (None, Some("value2"))]);
/// ```
pub fn verify_has_no_conflicting_options(option_pairs: Vec<(Option<&str>, Option<&str>)>) {
    for (opt1, opt2) in option_pairs {
        if opt1.is_some() && opt2.is_some() {
            eprintln!(
                "Error: Conflicting options provided: {:?}, {:?}",
                opt1.unwrap(),
                opt2.unwrap()
            );
            std::process::exit(1);
        }
    }
}
