use crate::handlers::build_command_handler::CommandBuilder;
use clap::{Arg, Command};

pub struct ExcludeMatchesArgBuilder;

impl CommandBuilder for ExcludeMatchesArgBuilder {
    /// Adds the `--exclude_matches` argument to the given command.
    ///
    /// The `--exclude_matches` argument is a flag that, when provided, sets its value to `true`.
    ///
    /// # Examples
    ///
    /// ```
    /// use clap::Command;
    /// use crate::arg_buliders::exclude_matches_arg_builder::ExcludeMatchesArgBuilder;
    /// use crate::handlers::build_command_handler::CommandBuilder;
    ///
    /// let mut cmd = Command::new("test_command");
    /// ExcludeMatchesArgBuilder::build(&mut cmd);
    ///
    /// // The `--exclude_matches` argument should be added to the command.
    /// let exclude_matches_arg = cmd.get_arguments().find(|arg| arg.get_id() == "exclude_matches");
    /// assert!(exclude_matches_arg.is_some(), "The 'exclude_matches' argument was not added.");
    ///
    /// // Verify that a boolean is retrieved from the `--exclude_matches` argument.
    /// let matches = cmd.clone().try_get_matches_from(vec!["test_command", "--exclude_matches"]).unwrap();
    /// assert!(matches.get_one::<bool>("exclude_matches").copied().unwrap_or(false), "The 'exclude_matches' argument did not return true.");
    /// ```
    fn build(cmd: &mut Command) {
        cmd.arg(
            Arg::new("exclude_matches")
                .long("exclude_matches")
                .help("Exclude all matches")
                .action(clap::ArgAction::SetTrue),
        );
    }
}
