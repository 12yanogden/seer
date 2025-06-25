use crate::handlers::build_command_handler::CommandBuilder;
use clap::{builder::NonEmptyStringValueParser, Arg, Command};

pub struct DirArgBuilder;

impl CommandBuilder for DirArgBuilder {
    /// Adds the `--dir` and `--max_depth` arguments to the command.
    ///
    /// The `--dir` argument requires a non-empty string value to be provided.
    ///
    /// # Examples
    ///
    /// ```
    /// use clap::Command;
    /// use crate::arg_buliders::dir_arg_builder::DirArgBuilder;
    /// use crate::handlers::build_command_handler::CommandBuilder;
    ///
    /// let mut cmd = Command::new("test_command");
    /// DirArgBuilder::build(&mut cmd);
    ///
    /// // The `--dir` argument should be added to the command.
    /// let dir_arg = cmd.get_arguments().find(|arg| arg.get_id() == "dir");
    /// assert!(dir_arg.is_some(), "The 'dir' argument was not added.");
    ///
    /// // The `--dir` argument should accept a non-empty string.
    /// let matches = cmd.try_get_matches_from(vec!["test_command", "--dir", "/path/to/dir"]);
    /// assert!(matches.is_ok(), "The 'dir' argument did not accept a valid value.");
    /// let matches = matches.unwrap();
    /// assert_eq!(
    ///     matches.get_one::<String>("dir").unwrap(),
    ///     "/path/to/dir",
    ///     "The 'dir' argument did not capture the correct value."
    /// );
    /// ```
    fn build(cmd: &mut Command) {
        cmd.arg(
            Arg::new("dir")
                .long("dir")
                .value_name("DIR_PATH")
                .value_parser(NonEmptyStringValueParser::new())
                .help("The directory to search within"),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Command;

    #[test]
    fn dir_argument_throws_error_without_a_string_given() {
        let mut cmd = Command::new("test_command");
        DirArgBuilder::build(&mut cmd);
        let matches = cmd.try_get_matches_from(vec!["test_command", "--dir"]);
        assert!(
            matches.is_err(),
            "The 'dir' argument did not require a value."
        );
    }
}
