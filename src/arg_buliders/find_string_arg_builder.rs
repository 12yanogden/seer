use crate::handlers::build_command_handler::CommandBuilder;
use clap::{builder::NonEmptyStringValueParser, Arg, Command};

pub struct FindStringArgBuilder;

impl CommandBuilder for FindStringArgBuilder {
    /// Adds the `--find_string` argument to the given command.
    ///
    /// The `--find_string` argument requires a non-empty string value to be provided.
    ///
    /// # Examples
    ///
    /// ```
    /// use clap::Command;
    /// use crate::arg_buliders::find_string_arg_builder::FindStringArgBuilder;
    /// use crate::handlers::build_command_handler::CommandBuilder;
    ///
    /// let mut cmd = Command::new("test_command");
    /// FindStringArgBuilder::build(&mut cmd);
    ///
    /// // The `--find_string` argument should be added to the command.
    /// let find_string_arg = cmd.get_arguments().find(|arg| arg.get_id() == "find_string");
    /// assert!(find_string_arg.is_some(), "The 'find_string' argument was not added.");
    ///
    /// // The `--find_string` argument should accept a string.
    /// let matches = cmd.try_get_matches_from(vec!["test_command", "--find_string", "string_value"]);
    /// assert!(matches.is_ok(), "The 'find_string' argument did not accept a value.");
    /// let matches = matches.unwrap();
    /// assert_eq!(
    ///     matches.get_one::<String>("find_string").unwrap(),
    ///     "string_value",
    ///     "The 'find_string' argument did not capture the correct value."
    /// );
    /// ```
    fn build(cmd: &mut Command) {
        cmd.arg(
            Arg::new("find_string")
                .long("find_string")
                .value_name("STRING_VALUE")
                .value_parser(NonEmptyStringValueParser::new())
                .help("Find matches using the given string value"),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Command;

    #[test]
    fn find_string_argument_throws_an_error_without_a_string_given() {
        let mut cmd = Command::new("test_command");
        FindStringArgBuilder::build(&mut cmd);

        // Test with no value
        let matches = cmd.try_get_matches_from(vec!["test_command", "--find_string"]);
        assert!(
            matches.is_err(),
            "The 'find_string' argument did not require a value."
        );
    }
}
