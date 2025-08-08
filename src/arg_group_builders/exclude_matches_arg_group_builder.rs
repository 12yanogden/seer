use crate::handlers::build_command_handler::CommandBuilder;
use clap::{ArgGroup, Command};

pub struct ExcludeMatchesArgGroupBuilder;

impl CommandBuilder for ExcludeMatchesArgGroupBuilder {
    /// Builds the `exclude_matches_arg_group` argument group and adds it to the given command.
    ///
    /// The group includes the `--exclude-matches` argument and requires the `--find-between` argument
    /// to be present when `--exclude-matches` is used.
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
    /// // Test with only --exclude-matches
    /// let matches = cmd.clone().try_get_matches_from(vec!["test_command", "--exclude-matches"]);
    /// assert!(matches.is_err());
    /// let err = matches.unwrap_err();
    /// assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);
    ///
    /// // Test with --find-between and --exclude-matches
    /// let matches = cmd.clone().try_get_matches_from(vec![
    ///     "test_command",
    ///     "--find-between",
    ///     "foo",
    ///     "bar",
    ///     "--exclude-matches",
    /// ]);
    /// assert!(matches.is_ok());
    /// ```
    fn build(cmd: &mut Command) {
        *cmd = cmd.clone().group(
            ArgGroup::new("exclude_matches_arg_group")
                .args(["exclude-matches"])
                .requires("find-between"),
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

        // Test with only --find-between
        let matches =
            cmd.clone()
                .try_get_matches_from(vec!["test_command", "--find-between", "foo", "bar"]);
        assert!(matches.is_ok());
    }
}
