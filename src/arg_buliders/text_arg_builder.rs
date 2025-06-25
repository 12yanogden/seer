use crate::handlers::build_command_handler::CommandBuilder;
use clap::{builder::NonEmptyStringValueParser, Arg, Command};

/// A builder for adding the `--text` argument to a command.
///
/// The `--text` argument requires a non-empty string value to be provided.
///
/// # Examples
///
/// ```
/// use clap::Command;
/// use crate::arg_buliders::text_arg_builder::TextArgBuilder;
/// use crate::handlers::build_command_handler::CommandBuilder;
///
/// let mut cmd = Command::new("test_command");
/// TextArgBuilder::build(&mut cmd);
///
/// // The `--text` argument should be added to the command.
/// let text_arg = cmd.get_arguments().find(|arg| arg.get_id() == "text");
/// assert!(text_arg.is_some(), "The 'text' argument was not added.");
///
/// // The `--text` argument should accept a string.
/// let matches = cmd.try_get_matches_from(vec!["test_command", "--text", "string_value"]);
/// assert!(matches.is_ok(), "The 'text' argument did not accept a value.");
/// let matches = matches.unwrap();
/// assert_eq!(
///     matches.get_one::<String>("text").unwrap(),
///     "string_value",
///     "The 'text' argument did not capture the correct value."
/// );
/// ```
pub struct TextArgBuilder;

impl CommandBuilder for TextArgBuilder {
    /// Adds the `--text` argument to the given command.
    ///
    /// The `--text` argument requires a non-empty string value to be provided.
    fn build(cmd: &mut Command) {
        cmd.arg(
            Arg::new("text")
                .long("text")
                .value_name("STRING_VALUE")
                .value_parser(NonEmptyStringValueParser::new())
                .help("Provide a text value for processing"),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Command;

    #[test]
    fn text_argument_throws_an_error_without_a_string_given() {
        let mut cmd = Command::new("test_command");
        TextArgBuilder::build(&mut cmd);

        // Test with no value
        let matches = cmd.try_get_matches_from(vec!["test_command", "--text"]);
        assert!(
            matches.is_err(),
            "The 'text' argument did not require a value."
        );
    }
}
