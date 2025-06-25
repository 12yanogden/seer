use crate::handlers::build_command_handler::CommandBuilder;
use clap::{builder::NonEmptyStringValueParser, Arg, Command};

pub struct FindRegexArgBuilder;

impl CommandBuilder for FindRegexArgBuilder {
    /// Adds the `--find_regex` argument to the given command.
    ///
    /// The `--find_regex` argument requires a non-empty string value to be provided.
    ///
    /// # Examples
    ///
    /// ```
    /// use clap::Command;
    /// use crate::arg_buliders::find_regex_arg_builder::FindRegexArgBuilder;
    /// use crate::handlers::build_command_handler::CommandBuilder;
    ///
    /// let mut cmd = Command::new("test_command");
    /// FindRegexArgBuilder::build(&mut cmd);
    ///
    /// // The `--find_regex` argument should be added to the command.
    /// let find_regex_arg = cmd.get_arguments().find(|arg| arg.get_id() == "find_regex");
    /// assert!(find_regex_arg.is_some(), "The 'find_regex' argument was not added.");
    ///
    /// // The `--find_regex` argument should accept a string.
    /// let matches = cmd.try_get_matches_from(vec!["test_command", "--find_regex", "regex_value"]);
    /// assert!(matches.is_ok(), "The 'find_regex' argument did not accept a value.");
    /// let matches = matches.unwrap();
    /// assert_eq!(
    ///     matches.get_one::<String>("find_regex").unwrap(),
    ///     "regex_value",
    ///     "The 'find_regex' argument did not capture the correct value."
    /// );
    /// ```
    fn build(cmd: &mut Command) {
        cmd.arg(
            Arg::new("find_regex")
                .long("find_regex")
                .value_name("REGEX_PATTERN")
                .value_parser(NonEmptyStringValueParser::new())
                .help("Find matches using the given regex pattern"),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Command;

    #[test]
    fn find_regex_argument_throws_an_error_without_a_string_given() {
        let mut cmd = Command::new("test_command");
        FindRegexArgBuilder::build(&mut cmd);

        // Test with no value
        let matches = cmd.try_get_matches_from(vec!["test_command", "--find_regex"]);
        assert!(
            matches.is_err(),
            "The 'find_regex' argument did not require a value."
        );
    }
}
