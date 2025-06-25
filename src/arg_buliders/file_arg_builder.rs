use crate::handlers::build_command_handler::CommandBuilder;
use clap::{builder::NonEmptyStringValueParser, Arg, Command};

pub struct FileArgBuilder;

impl CommandBuilder for FileArgBuilder {
    /// Adds the `--file` argument to the given command.
    ///
    /// The `--file` argument requires a non-empty string value representing a file path.
    ///
    /// # Examples
    ///
    /// ```
    /// use clap::Command;
    /// use crate::arg_buliders::file_arg_builder::FileArgBuilder;
    /// use crate::handlers::build_command_handler::CommandBuilder;
    ///
    /// let mut cmd = Command::new("test_command");
    /// FileArgBuilder::build(&mut cmd);
    ///
    /// // The `--file` argument should be added to the command.
    /// let file_arg = cmd.get_arguments().find(|arg| arg.get_id() == "file");
    /// assert!(file_arg.is_some(), "The 'file' argument was not added.");
    ///
    /// // The `--file` argument should accept a valid file path.
    /// let matches = cmd.try_get_matches_from(vec!["test_command", "--file", "/path/to/file"]);
    /// assert!(matches.is_ok(), "The 'file' argument did not accept a valid value.");
    /// let matches = matches.unwrap();
    /// assert_eq!(
    ///     matches.get_one::<String>("file").unwrap(),
    ///     "/path/to/file",
    ///     "The 'file' argument did not capture the correct value."
    /// );
    /// ```
    fn build(cmd: &mut Command) {
        cmd.arg(
            Arg::new("file")
                .long("file")
                .value_name("FILE_PATH")
                .value_parser(NonEmptyStringValueParser::new())
                .help("Specify the file path to process"),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Command;

    #[test]
    fn file_argument_throws_error_without_a_string_given() {
        let mut cmd = Command::new("test_command");
        FileArgBuilder::build(&mut cmd);
        let matches = cmd.try_get_matches_from(vec!["test_command", "--file"]);
        assert!(
            matches.is_err(),
            "The 'file' argument did not require a value."
        );
    }
}
