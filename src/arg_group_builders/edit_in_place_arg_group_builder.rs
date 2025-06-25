use crate::handlers::build_command_handler::CommandBuilder;
use clap::{ArgGroup, Command};

pub struct EditInPlaceArgGroupBuilder;

impl CommandBuilder for EditInPlaceArgGroupBuilder {
    /// Builds the `edit_in_place_arg_group` argument group and adds it to the given command.
    ///
    /// The group includes the `--edit_in_place` argument and requires one of the `--dir`, `--file`,
    /// or `--files` arguments to be present when `--edit_in_place` is used.
    ///
    /// # Arguments
    ///
    /// * `cmd` - The command to which the argument group will be added.
    ///
    /// # Returns
    ///
    /// The command with the `edit_in_place_arg_group` added.
    ///
    /// # Examples
    ///
    /// ```
    /// use clap::{Command, ArgGroup, error::ErrorKind};
    /// use crate::arg_group_builders::edit_in_place_arg_group_builder::EditInPlaceArgGroupBuilder;
    /// use crate::handlers::build_command_handler::CommandBuilder;
    ///
    /// let mut cmd = Command::new("test_command");
    /// EditInPlaceArgGroupBuilder::build(&mut cmd);
    ///
    /// // Test with only --edit_in_place
    /// let matches = cmd.clone().try_get_matches_from(vec!["test_command", "--edit_in_place"]);
    /// assert!(matches.is_err());
    /// let err = matches.unwrap_err();
    /// assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);
    ///
    /// // Test with --edit_in_place and --dir
    /// let matches = cmd.clone().try_get_matches_from(vec![
    ///     "test_command",
    ///     "--edit_in_place",
    ///     "--dir",
    ///     "some_dir",
    /// ]);
    /// assert!(matches.is_ok());
    /// ```
    fn build(cmd: &mut Command) {
        cmd.group(
            ArgGroup::new("edit_in_place_arg_group")
                .args(["edit_in_place"])
                .requires_all(&["dir", "file", "files"]),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Command;

    lazy_static::lazy_static! {
        static ref CMD: Command = {
            let mut cmd = Command::new("test_command");
            EditInPlaceArgGroupBuilder::build(&mut cmd);
            cmd
        };
    }

    #[test]
    fn test_edit_in_place_allowed_with_file() {
        let matches = CMD.clone().try_get_matches_from(vec![
            "test_command",
            "--edit_in_place",
            "--file",
            "some_file",
        ]);
        assert!(matches.is_ok());
    }

    #[test]
    fn test_edit_in_place_allowed_with_files() {
        let matches = CMD.clone().try_get_matches_from(vec![
            "test_command",
            "--edit_in_place",
            "--files",
            "file1",
            "file2",
        ]);
        assert!(matches.is_ok());
    }
}
