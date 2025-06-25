use crate::handlers::build_command_handler::CommandBuilder;
use clap::{builder::NonEmptyStringValueParser, Arg, Command};

pub struct ReplaceWithArgBuilder;

impl CommandBuilder for ReplaceWithArgBuilder {
    /// Adds the `--replace_with` argument to the given command.
    ///
    /// The `--replace_with` argument requires a non-empty string value to be provided.
    ///
    /// # Examples
    ///
    /// ```
    /// use clap::Command;
    /// use crate::arg_buliders::replace_arg_builder::ReplaceWithArgBuilder;
    /// use crate::handlers::build_command_handler::CommandBuilder;
    ///
    /// let mut cmd = Command::new("test_command");
    /// ReplaceWithArgBuilder::build(&mut cmd);
    ///
    /// // The `--replace_with` argument should be added to the command.
    /// let replace_with_arg = cmd.get_arguments().find(|arg| arg.get_id() == "replace_with");
    /// assert!(replace_with_arg.is_some(), "The 'replace_with' argument was not added.");
    ///
    /// // The `--replace_with` argument should accept a string.
    /// let matches = cmd.try_get_matches_from(vec!["test_command", "--replace_with", "replacement_string"]);
    /// assert!(matches.is_ok(), "The 'replace_with' argument did not accept a value.");
    /// let matches = matches.unwrap();
    /// assert_eq!(
    ///     matches.get_one::<String>("replace_with").unwrap(),
    ///     "replacement_string",
    ///     "The 'replace_with' argument did not capture the correct value."
    /// );
    /// ```
    fn build(cmd: &mut Command) {
        cmd.arg(
            Arg::new("replace_with")
                .long("replace_with")
                .value_name("REPLACEMENT_STRING")
                .value_parser(NonEmptyStringValueParser::new())
                .help("Replace matches using the given string value"),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Command;

    #[test]
    fn replace_with_argument_throws_an_error_without_a_string_given() {
        let mut cmd = Command::new("test_command");
        ReplaceWithArgBuilder::build(&mut cmd);

        // Test with no value
        let matches = cmd.try_get_matches_from(vec!["test_command", "--replace_with"]);
        assert!(
            matches.is_err(),
            "The 'replace_with' argument did not require a value."
        );
    }
}
