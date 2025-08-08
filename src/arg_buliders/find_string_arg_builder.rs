use crate::handlers::build_command_handler::CommandBuilder;
use clap::{builder::NonEmptyStringValueParser, Arg, Command};

pub struct FindStringArgBuilder;

impl CommandBuilder for FindStringArgBuilder {
    /// Adds the `--find-string` argument to the given command.
    ///
    /// The `--find-string` argument requires a non-empty string value to be provided.
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
    /// // The `--find-string` argument should be added to the command.
    /// let find_string_arg = cmd.get_arguments().find(|arg| arg.get_id() == "find-string");
    /// assert!(find_string_arg.is_some(), "The 'find-string' argument was not added.");
    ///
    /// // The `--find-string` argument should accept a string.
    /// let matches = cmd.try_get_matches_from(vec!["test_command", "--find-string", "string_value"]);
    /// assert!(matches.is_ok(), "The 'find-string' argument did not accept a value.");
    /// let matches = matches.unwrap();
    /// assert_eq!(
    ///     matches.get_one::<String>("find-string").unwrap(),
    ///     "string_value",
    ///     "The 'find-string' argument did not capture the correct value."
    /// );
    /// ```
    fn build(cmd: &mut Command) {
        *cmd = cmd.clone().arg(
            Arg::new("find-string")
                .long("find-string")
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
        let matches = cmd.try_get_matches_from(vec!["test_command", "--find-string"]);
        assert!(
            matches.is_err(),
            "The 'find-string' argument did not require a value."
        );
    }
}
