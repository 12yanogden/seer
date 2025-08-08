use crate::handlers::build_command_handler::CommandBuilder;
use clap::{ArgGroup, Command};

pub struct EditInPlaceArgGroupBuilder;

impl CommandBuilder for EditInPlaceArgGroupBuilder {
    /// Builds the `edit_in_place_arg_group` argument group and adds it to the given command.
    ///
    /// The `--edit-in-place` argument requires:
    /// 1. At least one edit operation argument: `--append`, `--prepend`, or `--replace-with`
    /// 2. At least one target argument: `--dir`, `--file`, or `--files`
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
    /// // Test with only --edit-in-place (should fail)
    /// let matches = cmd.clone().try_get_matches_from(vec!["test_command", "--edit-in-place"]);
    /// assert!(matches.is_err());
    /// let err = matches.unwrap_err();
    /// assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);
    ///
    /// // Test with --edit-in-place, --append, and --dir (should pass)
    /// let matches = cmd.clone().try_get_matches_from(vec![
    ///     "test_command",
    ///     "--edit-in-place",
    ///     "--append",
    ///     "text",
    ///     "--dir",
    ///     "some_dir",
    /// ]);
    /// assert!(matches.is_ok());
    /// ```
    fn build(cmd: &mut Command) {
        // Create target group - when edit-in-place is used, require at least one target
        *cmd = cmd.clone().group(
            ArgGroup::new("edit_target_group")
                .args(&["dir", "file", "files"])
                .required(false)
                .multiple(false),
        );
        
        // Create edit operation group - when edit-in-place is used, require at least one edit operation
        *cmd = cmd.clone().group(
            ArgGroup::new("edit_operation_group")
                .args(&["append", "prepend", "replace-with"])
                .required(false)
                .multiple(false),
        );
        
        // Create a group that makes edit-in-place conditional on having both
        // a target and an edit operation. When edit-in-place is present, both groups become required.
        *cmd = cmd.clone().group(
            ArgGroup::new("edit_in_place_requirements")
                .args(&["edit-in-place"])
                .requires("edit_target_group")
                .requires("edit_operation_group")
                .required(false),
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
    fn test_edit_in_place_fails_without_edit_operation() {
        let matches = CMD.clone().try_get_matches_from(vec![
            "test_command",
            "--edit-in-place",
            "--file",
            "some_file",
        ]);
        assert!(matches.is_err(), "edit-in-place should require an edit operation");
    }

    #[test]
    fn test_edit_in_place_fails_without_target() {
        let matches = CMD.clone().try_get_matches_from(vec![
            "test_command",
            "--edit-in-place",
            "--append",
            "text",
        ]);
        assert!(matches.is_err(), "edit-in-place should require a target argument");
    }

    #[test]
    fn test_edit_in_place_allowed_with_file_and_append() {
        let matches = CMD.clone().try_get_matches_from(vec![
            "test_command",
            "--edit-in-place",
            "--append",
            "text",
            "--file",
            "some_file",
        ]);
        assert!(matches.is_ok());
    }

    #[test]
    fn test_edit_in_place_allowed_with_files_and_prepend() {
        let matches = CMD.clone().try_get_matches_from(vec![
            "test_command",
            "--edit-in-place",
            "--prepend",
            "prefix",
            "--files",
            "file1",
            "file2",
        ]);
        assert!(matches.is_ok());
    }

    #[test]
    fn test_edit_in_place_allowed_with_dir_and_replace_with() {
        let matches = CMD.clone().try_get_matches_from(vec![
            "test_command",
            "--edit-in-place",
            "--replace-with",
            "replacement",
            "--dir",
            "some_dir",
        ]);
        assert!(matches.is_ok());
    }

    #[test]
    fn test_edit_in_place_fails_with_only_flag() {
        let matches = CMD.clone().try_get_matches_from(vec![
            "test_command",
            "--edit-in-place",
        ]);
        assert!(matches.is_err(), "edit-in-place should fail when used alone");
    }
}
