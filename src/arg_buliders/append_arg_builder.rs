use crate::handlers::build_command_handler::CommandBuilder;
use clap::{builder::NonEmptyStringValueParser, Arg, Command};

pub struct AppendArgBuilder;

impl CommandBuilder for AppendArgBuilder {
    /// Adds the `--append` argument to the given command.
    ///
    /// The `--append` argument requires a non-empty string value to be provided.
    ///
    /// # Examples
    ///
    /// ```
    /// use clap::Command;
    /// use crate::arg_buliders::append_arg_builder::AppendArgBuilder;
    /// use crate::handlers::build_command_handler::CommandBuilder;
    ///
    /// let mut cmd = Command::new("test_command");
    /// AppendArgBuilder::build(&mut cmd);
    ///
    /// // The `--append` argument should be added to the command.
    /// let append_arg = cmd.get_arguments().find(|arg| arg.get_id() == "append");
    /// assert!(append_arg.is_some(), "The 'append' argument was not added.");
    ///
    /// // The `--append` argument should accept a string.
    /// let matches = cmd.try_get_matches_from(vec!["test_command", "--append", "value"]);
    /// assert!(matches.is_ok(), "The 'append' argument did not accept a value.");
    /// let matches = matches.unwrap();
    /// assert_eq!(
    ///     matches.get_one::<String>("append").unwrap(),
    ///     "value",
    ///     "The 'append' argument did not capture the correct value."
    /// );
    /// ```
    fn build(cmd: &mut Command) {
        cmd.arg(
            Arg::new("append")
                .long("append")
                .value_name("STRING_TO_APPEND")
                .value_parser(NonEmptyStringValueParser::new())
                .help("Append the string given to match(es)"),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Command;

    #[test]
    fn append_argument_throws_error_without_a_string_given() {
        let mut cmd = Command::new("test_command");
        AppendArgBuilder::build(&mut cmd);
        let matches = cmd.try_get_matches_from(vec!["test_command", "--append"]);
        assert!(
            matches.is_err(),
            "The 'append' argument did not require a value."
        );
    }
}
