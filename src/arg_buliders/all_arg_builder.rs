use crate::handlers::build_command_handler::CommandBuilder;
use clap::{Arg, Command};

pub struct AllArgBuilder;

impl CommandBuilder for AllArgBuilder {
    /// Adds the `--all` argument to the given command.
    ///
    /// The `--all` argument is a flag that, when provided, sets its value to `true`.
    ///
    /// # Examples
    ///
    /// ```
    /// use clap::Command;
    /// use crate::arg_buliders::all_arg_builder::AllArgBuilder;
    /// use crate::handlers::build_command_handler::CommandBuilder;
    ///
    /// let cmd = Command::new("test_command");
    /// AllArgBuilder::build(&mut cmd);
    ///
    /// // The `--all` argument should be added to the command.
    /// let all_arg = cmd.get_arguments().find(|arg| arg.get_id() == "all");
    /// assert!(all_arg.is_some(), "The 'all' argument was not added.");
    ///
    /// // Verify that a boolean is retrieved from the `--all` argument.
    /// let matches = cmd.try_get_matches_from(vec!["test_command", "--all"]).unwrap();
    /// assert!(matches.get_one::<bool>("all").copied().unwrap_or(false), "The 'all' argument did not return true.");
    /// ```
    fn build(&mut cmd: Command) {
        cmd.arg(
            Arg::new("all")
                .long("all")
                .help("Find all matches")
                .action(clap::ArgAction::SetTrue),
        )
    }
}
