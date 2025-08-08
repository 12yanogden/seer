use crate::handlers::build_command_handler::CommandBuilder;
use clap::{Arg, Command};

pub struct ExcludeMatchesArgBuilder;

impl CommandBuilder for ExcludeMatchesArgBuilder {
    /// Adds the `--exclude-matches` argument to the given command.
    ///
    /// The `--exclude-matches` argument is a flag that, when provided, sets its value to `true`.
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
    /// // The `--exclude-matches` argument should be added to the command.
    /// let exclude_matches_arg = cmd.get_arguments().find(|arg| arg.get_id() == "exclude-matches");
    /// assert!(exclude_matches_arg.is_some(), "The 'exclude-matches' argument was not added.");
    ///
    /// // Verify that a boolean is retrieved from the `--exclude-matches` argument.
    /// let matches = cmd.clone().try_get_matches_from(vec!["test_command", "--exclude-matches"]).unwrap();
    /// assert!(matches.get_one::<bool>("exclude-matches").copied().unwrap_or(false), "The 'exclude-matches' argument did not return true.");
    /// ```
    fn build(cmd: &mut Command) {
        *cmd = cmd.clone().arg(
            Arg::new("exclude-matches")
                .long("exclude-matches")
                .help("Exclude all matches")
                .action(clap::ArgAction::SetTrue),
        );
    }
}
