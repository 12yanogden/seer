use crate::handlers::build_command_handler::CommandBuilder;
use clap::{Arg, Command};

pub struct FilesArgBuilder;

impl CommandBuilder for FilesArgBuilder {
    /// Adds the `--files` argument to the given command.
    ///
    /// The `--files` argument accepts a comma-separated list of file paths.
    ///
    /// # Examples
    ///
    /// ```
    /// use clap::Command;
    /// use crate::arg_buliders::files_arg_builder::FilesArgBuilder;
    /// use crate::handlers::build_command_handler::CommandBuilder;
    ///
    /// let mut cmd = Command::new("test_command");
    /// FilesArgBuilder::build(&mut cmd);
    ///
    /// // The `--files` argument should be added to the command.
    /// let files_arg = cmd.get_arguments().find(|arg| arg.get_id() == "files");
    /// assert!(files_arg.is_some(), "The 'files' argument was not added.");
    ///
    /// // The `--files` argument should accept multiple file paths.
    /// let matches = cmd.try_get_matches_from(vec!["test_command", "--files", "/path/one,/path/two"]);
    /// assert!(matches.is_ok(), "The 'files' argument did not accept valid values.");
    /// let matches = matches.unwrap();
    /// let files: Vec<&String> = matches.get_many::<String>("files").unwrap().collect();
    /// assert_eq!(
    ///     files,
    ///     vec!["/path/one", "/path/two"],
    ///     "The 'files' argument did not capture the correct values."
    /// );
    /// ```
    fn build(cmd: &mut Command) {
        cmd.arg(
            Arg::new("files")
                .long("files")
                .value_name("FILES")
                .value_delimiter(',')
                .num_args(1..)
                .help("An array of file paths to search within"),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Command;

    #[test]
    fn files_argument_throws_error_without_at_least_one_string_given() {
        let mut cmd = Command::new("test_command");
        FilesArgBuilder::build(&mut cmd);
        let matches = cmd.try_get_matches_from(vec!["test_command", "--files"]);
        assert!(
            matches.is_err(),
            "The 'files' argument did not require at least one value."
        );
    }
}
