use crate::handlers::build_command_handler::CommandBuilder;
use clap::{Arg, Command};

pub struct EditInPlaceArgBuilder;

impl CommandBuilder for EditInPlaceArgBuilder {
    /// Adds the `--edit_in_place` argument to the given command.
    ///
    /// The `--edit_in_place` argument is a flag that, when provided, sets its value to `true`.
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
    /// // The `--edit_in_place` argument should be added to the command.
    /// let edit_in_place_arg = cmd.get_arguments().find(|arg| arg.get_id() == "edit_in_place");
    /// assert!(edit_in_place_arg.is_some(), "The 'edit_in_place' argument was not added.");
    ///
    /// // Verify that a boolean is retrieved from the `--edit_in_place` argument.
    /// let matches = cmd.clone().try_get_matches_from(vec!["test_command", "--edit_in_place"]).unwrap();
    /// assert!(matches.get_one::<bool>("edit_in_place").copied().unwrap_or(false), "The 'edit_in_place' argument did not return true.");
    /// ```
    fn build(cmd: &mut Command) {
        cmd.arg(
            Arg::new("edit_in_place")
                .long("edit_in_place")
                .help("Edit files in place")
                .action(clap::ArgAction::SetTrue),
        );
    }
}
