use crate::handlers::build_command_handler::CommandBuilder;
use clap::{ArgGroup, Command};

pub struct ExcludeMatchesArgGroupBuilder;

impl CommandBuilder for ExcludeMatchesArgGroupBuilder {
    /// Builds the `exclude_matches_arg_group` argument group and adds it to the given command.
    ///
    /// The group includes the `--exclude_matches` argument and requires the `--find_between` argument
    /// to be present when `--exclude_matches` is used.
    ///
    /// # Arguments
    ///
    /// * `cmd` - The command to which the argument group will be added.
    ///
    /// # Returns
    ///
    /// The command with the `exclude_matches_arg_group` added.
    ///
    /// # Examples
    ///
    /// ```
    /// use clap::{Command, ArgGroup, error::ErrorKind};
    /// use crate::arg_group_builders::exclude_matches_arg_group_builder::ExcludeMatchesArgGroupBuilder;
    /// use crate::handlers::build_command_handler::CommandBuilder;
    ///
    /// let mut cmd = Command::new("test_command");
    /// ExcludeMatchesArgGroupBuilder::build(&mut cmd);
    ///
    /// // Verify that the group is added to the command.
    /// let group = cmd.get_groups().find(|group| group.get_id() == "exclude_matches_arg_group");
    /// assert!(group.is_some(), "The 'exclude_matches_arg_group' was not added to the command.");
    ///
    /// // Test with only --exclude_matches
    /// let matches = cmd.clone().try_get_matches_from(vec!["test_command", "--exclude_matches"]);
    /// assert!(matches.is_err());
    /// let err = matches.unwrap_err();
    /// assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);
    ///
    /// // Test with --find_between and --exclude_matches
    /// let matches = cmd.clone().try_get_matches_from(vec![
    ///     "test_command",
    ///     "--find_between",
    ///     "foo",
    ///     "bar",
    ///     "--exclude_matches",
    /// ]);
    /// assert!(matches.is_ok());
    /// ```
    fn build(cmd: &mut Command) {
        cmd.group(
            ArgGroup::new("exclude_matches_arg_group")
                .args(["exclude_matches"])
                .requires("find_between"),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Command;

    #[test]
    fn test_find_between_argument() {
        let mut cmd = Command::new("test_command");
        ExcludeMatchesArgGroupBuilder::build(&mut cmd);

        // Test with only --find_between
        let matches =
            cmd.clone()
                .try_get_matches_from(vec!["test_command", "--find_between", "foo", "bar"]);
        assert!(matches.is_ok());
    }
}
