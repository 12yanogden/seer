use clap::{Arg, ArgGroup, Command};

/// Parses command line arguments using the `clap` crate.
///
/// # Returns
///
/// A `Command` instance containing the parsed arguments.
///
/// # Examples
///
/// ```
/// use seek::init;
/// use clap::CommandFactory;
///
/// let cmd = seek::init();
/// let matches = cmd.override_usage("seek --regex <REGEX> --text <TEXT>")
///     .try_get_matches_from(vec!["seek", "--regex", "foo", "--text", "test string"])
///     .unwrap();
/// assert_eq!(matches.get_one::<String>("regex").unwrap(), "foo");
/// assert_eq!(matches.get_one::<String>("text").unwrap(), "test string");
/// ```
pub fn init() -> Command {
    Command::new("seek")
        .version("1.0")
        .author("Ryan Ogden <12yanogden@gmail.com>")
        .about("Find/edit matches in a string or file using plain text or regex patterns.")

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
                .help("Two regex patterns to search for. Find/edit both matches and the text between. Use --exclude_matches to only find/edit the text between.")
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
                .help("Only find/edit the text between the two regex patterns")
                .action(clap::ArgAction::SetTrue),
        )
        .group(
            ArgGroup::new("between_group")
                .args(["exclude_matches"])
                .requires("between"),
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
        .arg(
            Arg::new("dir")
                .long("dir")
                .value_name("DIR")
                .help("The directory to search within")
                .action(clap::ArgAction::Set),
        )
        .group(
            ArgGroup::new("searchable")
                .args(["text", "file", "dir"])
                .required(false)
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

        // Dir group
        .arg(
            Arg::new("max_depth")
                .long("max_depth")
                .value_name("MAX_DEPTH")
                .help("The maximum depth to search within the directory")
                .value_parser(clap::value_parser!(u64).range(0..))
                .action(clap::ArgAction::Set),
        )
        .group(
            ArgGroup::new("dir_group")
                .args(["dir", "max_depth"])
                .required(false)
                .requires("dir"),
        )

        // Frequency group
        .arg(
            Arg::new("nth")
                .long("nth")
                .value_name("NTH")
                .help("Find/edit only the nth match")
                .value_parser(clap::value_parser!(u64).range(0..))
                .action(clap::ArgAction::Set),
        )
        .arg(
            Arg::new("every_nth")
                .long("every_nth")
                .value_name("EVERY_NTH")
                .help("Find/edit every nth match")
                .value_parser(clap::value_parser!(u64).range(0..))
                .action(clap::ArgAction::Set),
        )
        .arg(
            Arg::new("all")
                .long("all")
                .value_name("ALL")
                .help("Find/edit all matches")
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
mod init_tests {
    use super::*;
    use clap::error::ErrorKind;

    lazy_static::lazy_static! {
        static ref CMD: Command = init();
    }

    #[test]
    fn test_one_and_only_one_search_strategy_can_be_given() {
        // Test with no search strategy (should fail)
        let matches = CMD.clone().try_get_matches_from(vec![
            "seek",
            "--text", "foo"
        ]);
        assert!(matches.is_err());
        let err = matches.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);

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
    #[ignore = "Blocked by clap issue https://github.com/clap-rs/clap/issues/4707"]
    fn test_exclude_matches_flag_requires_between_flag() {
        // Test with only --exclude_matches (should fail)
        let matches = CMD.clone().try_get_matches_from(vec![
            "seek",
            "--target", "foo",
            "--text", "bar",
            "--exclude_matches"
        ]);
        assert!(matches.is_err());
        let err = matches.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);

        // Test with --between and --exclude_matches (should pass)
        let matches = CMD.clone().try_get_matches_from(vec![
            "seek",
            "--between", "foo", "bar",
            "--text", "bar",
            "--exclude_matches"
        ]);
        assert!(matches.is_ok());
    }

    #[test]
    fn test_zero_or_one_edit_strategy_can_be_given() {
        // Test with no edit strategy
        let matches = CMD.clone().try_get_matches_from(vec![
            "seek",
            "--target", "foo",
            "--text", "bar",
        ]);
        assert!(matches.is_ok());

        // Test with only --prepend
        let matches = CMD.clone().try_get_matches_from(vec![
            "seek",
            "--target", "foo",
            "--text", "bar",
            "--prepend", "baz"
        ]);
        assert!(matches.is_ok());

        // Test with only --replace_with
        let matches = CMD.clone().try_get_matches_from(vec![
            "seek",
            "--target", "foo",
            "--text", "bar",
            "--replace_with", "baz"
        ]);
        assert!(matches.is_ok());

        // Test with only --append
        let matches = CMD.clone().try_get_matches_from(vec![
            "seek",
            "--target", "foo",
            "--text", "bar",
            "--append", "baz"
        ]);
        assert!(matches.is_ok());

        // Test with --prepend and --replace_with (should fail)
        let matches = CMD.clone().try_get_matches_from(vec![
            "seek",
            "--target", "foo",
            "--text", "bar",
            "--prepend", "baz",
            "--replace_with", "qux"
        ]);
        assert!(matches.is_err());
        let err = matches.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::ArgumentConflict);

        // Test with --prepend and --append (should fail)
        let matches = CMD.clone().try_get_matches_from(vec![
            "seek",
            "--target", "foo",
            "--text", "bar",
            "--prepend", "baz",
            "--append", "qux"
        ]);
        assert!(matches.is_err());
        let err = matches.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::ArgumentConflict);

        // Test with --replace_with and --append (should fail)
        let matches = CMD.clone().try_get_matches_from(vec![
            "seek",
            "--target", "foo",
            "--text", "bar",
            "--replace_with", "baz",
            "--append", "qux"
        ]);
        assert!(matches.is_err());
        let err = matches.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::ArgumentConflict);
    }

    #[test]
    fn test_zero_or_one_searchable_can_be_given() {
        // Test with no searchable (should fail)
        let matches = CMD.clone().try_get_matches_from(vec![
            "seek",
            "--target", "foo"
        ]);
        assert!(matches.is_ok());

        // Test with only --text
        let matches = CMD.clone().try_get_matches_from(vec![
            "seek",
            "--target", "foo",
            "--text", "bar"
        ]);
        assert!(matches.is_ok());

        // Test with only --file
        let matches = CMD.clone().try_get_matches_from(vec![
            "seek",
            "--target", "foo",
            "--file", "path/to/file"
        ]);
        assert!(matches.is_ok());

        // Test with only --dir
        let matches = CMD.clone().try_get_matches_from(vec![
            "seek",
            "--target", "foo",
            "--dir", "path/to/dir"
        ]);
        assert!(matches.is_ok());

        // Test with --text and --file (should fail)
        let matches = CMD.clone().try_get_matches_from(vec![
            "seek",
            "--target", "foo",
            "--text", "bar",
            "--file", "path/to/file"
        ]);
        assert!(matches.is_err());
        let err = matches.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::ArgumentConflict);

        // Test with --text and --dir (should fail)
        let matches = CMD.clone().try_get_matches_from(vec![
            "seek",
            "--target", "foo",
            "--text", "bar",
            "--dir", "path/to/dir"
        ]);
        assert!(matches.is_err());
        let err = matches.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::ArgumentConflict);

        // Test with --file and --dir (should fail)
        let matches = CMD.clone().try_get_matches_from(vec![
            "seek",
            "--target", "foo",
            "--file", "path/to/file",
            "--dir", "path/to/dir"
        ]);
        assert!(matches.is_err());
        let err = matches.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::ArgumentConflict);
    }

    #[test]
    #[ignore = "Blocked by clap issue https://github.com/clap-rs/clap/issues/4707"]
    fn test_in_place_flag_requires_file_flag() {
        // Test with only --in_place (should fail)
        let matches = CMD.clone().try_get_matches_from(vec![
            "seek",
            "--target", "foo",
            "--text", "bar",
            "--in_place"
        ]);
        assert!(matches.is_err());
        let err = matches.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);

        // Test with --file and --in_place (should pass)
        let matches = CMD.clone().try_get_matches_from(vec![
            "seek",
            "--target", "foo",
            "--file", "path/to/file",
            "--in_place"
        ]);
        assert!(matches.is_ok());
    }

    #[test]
    #[ignore = "Blocked by clap issue https://github.com/clap-rs/clap/issues/4707"]
    fn test_max_depth_flag_requires_dir_flag() {
        // Test with only --max_depth (should fail)
        let matches = CMD.clone().try_get_matches_from(vec![
            "seek",
            "--target", "foo",
            "--text", "bar",
            "--max_depth", "2"
        ]);
        assert!(matches.is_err());
        let err = matches.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);

        // Test with --dir and --max_depth (should pass)
        let matches = CMD.clone().try_get_matches_from(vec![
            "seek",
            "--target", "foo",
            "--dir", "path/to/dir",
            "--max_depth", "2"
        ]);
        assert!(matches.is_ok());
    }

    #[test]
    fn test_zero_or_one_frequencies_can_be_given() {
        // Test with no frequency
        let matches = CMD.clone().try_get_matches_from(vec![
            "seek",
            "--target", "foo",
            "--text", "bar"
        ]);
        assert!(matches.is_ok());

        // Test with only --nth
        let matches = CMD.clone().try_get_matches_from(vec![
            "seek",
            "--target", "foo",
            "--text", "bar",
            "--nth", "1"
        ]);
        assert!(matches.is_ok());

        // Test with only --every_nth
        let matches = CMD.clone().try_get_matches_from(vec![
            "seek",
            "--target", "foo",
            "--text", "bar",
            "--every_nth", "2"
        ]);
        assert!(matches.is_ok());

        // Test with only --all
        let matches = CMD.clone().try_get_matches_from(vec![
            "seek",
            "--target", "foo",
            "--text", "bar",
            "--all"
        ]);
        assert!(matches.is_ok());

        // Test with --nth and --every_nth (should fail)
        let matches = CMD.clone().try_get_matches_from(vec![
            "seek",
            "--target", "foo",
            "--text", "bar",
            "--nth", "1",
            "--every_nth", "2"
        ]);
        assert!(matches.is_err());
        let err = matches.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::ArgumentConflict);

        // Test with --nth and --all (should fail)
        let matches = CMD.clone().try_get_matches_from(vec![
            "seek",
            "--target", "foo",
            "--text", "bar",
            "--nth", "1",
            "--all"
        ]);
        assert!(matches.is_err());
        let err = matches.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::ArgumentConflict);

        // Test with --every_nth and --all (should fail)
        let matches = CMD.clone().try_get_matches_from(vec![
            "seek",
            "--target", "foo",
            "--text", "bar",
            "--every_nth", "2",
            "--all"
        ]);
        assert!(matches.is_err());
        let err = matches.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::ArgumentConflict);
    }
}