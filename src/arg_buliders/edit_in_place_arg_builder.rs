use crate::handlers::build_command_handler::CommandBuilder;
use clap::{Arg, Command};

pub struct EditInPlaceArgBuilder;

impl CommandBuilder for EditInPlaceArgBuilder {
    /// Adds the `--edit-in-place` argument to the given command.
    ///
    /// The `--edit-in-place` argument requires at least one edit operation argument
    /// (--append, --prepend, or --replace-with) and at least one target argument
    /// (--dir, --file, or --files) to be provided.
    ///
    /// # Examples
    ///
    /// ```
    /// use clap::Command;
    /// use crate::arg_buliders::edit_in_place_arg_builder::EditInPlaceArgBuilder;
    /// use crate::handlers::build_command_handler::CommandBuilder;
    ///
    /// let mut cmd = Command::new("test_command");
    /// EditInPlaceArgBuilder::build(&mut cmd);
    ///
    /// // The `--edit-in-place` argument should be added to the command.
    /// let edit_in_place_arg = cmd.get_arguments().find(|arg| arg.get_id() == "edit-in-place");
    /// assert!(edit_in_place_arg.is_some(), "The 'edit-in-place' argument was not added.");
    ///
    /// // Using --edit-in-place alone should fail (requires dependencies)
    /// let matches = cmd.clone().try_get_matches_from(vec!["test_command", "--edit-in-place"]);
    /// assert!(matches.is_err(), "edit-in-place should require dependencies");
    /// ```
    fn build(cmd: &mut Command) {
        *cmd = cmd.clone().arg(
            Arg::new("edit-in-place")
                .long("edit-in-place")
                .help("Edit files in place (requires at least one edit operation and one target)")
                .action(clap::ArgAction::SetTrue),
        );
    }
}
