use crate::handlers::build_command_handler::CommandBuilder;
use clap::{builder::NonEmptyStringValueParser, Arg, Command};

pub struct PrependArgBuilder;

impl CommandBuilder for PrependArgBuilder {
    /// Adds the `--prepend` argument to the given command.
    ///
    /// The `--prepend` argument requires a non-empty string value to be provided.
    ///
    /// # Examples
    ///
    /// ```
    /// use clap::Command;
    /// use crate::arg_buliders::prepend_arg_builder::PrependArgBuilder;
    /// use crate::handlers::build_command_handler::CommandBuilder;
    ///
    /// let mut cmd = Command::new("test_command");
    /// PrependArgBuilder::build(&mut cmd);
    ///
    /// // The `--prepend` argument should be added to the command.
    /// let prepend_arg = cmd.get_arguments().find(|arg| arg.get_id() == "prepend");
    /// assert!(prepend_arg.is_some(), "The 'prepend' argument was not added.");
    ///
    /// // The `--prepend` argument should accept a string.
    /// let matches = cmd.try_get_matches_from(vec!["test_command", "--prepend", "value"]);
    /// assert!(matches.is_ok(), "The 'prepend' argument did not accept a value.");
    /// let matches = matches.unwrap();
    /// assert_eq!(
    ///     matches.get_one::<String>("prepend").unwrap(),
    ///     "value",
    ///     "The 'prepend' argument did not capture the correct value."
    /// );
    /// ```
    fn build(cmd: &mut Command) {
        cmd.arg(
            Arg::new("prepend")
                .long("prepend")
                .value_name("STRING_TO_PREPEND")
                .value_parser(NonEmptyStringValueParser::new())
                .help("Prepend the string given to match(es)"),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Command;

    #[test]
    fn prepend_argument_throws_error_without_a_string_given() {
        let mut cmd = Command::new("test_command");
        PrependArgBuilder::build(&mut cmd);
        let matches = cmd.try_get_matches_from(vec!["test_command", "--prepend"]);
        assert!(
            matches.is_err(),
            "The 'prepend' argument did not require a value."
        );
    }
}
