use crate::handlers::build_command_handler::CommandBuilder;
use clap::{Arg, Command};

pub struct CountArgBuilder;

impl CommandBuilder for CountArgBuilder {
    /// Adds the `--count` argument to the given command.
    ///
    /// The `--count` argument is a flag that, when provided, sets its value to `true`.
    ///
    /// # Examples
    ///
    /// ```
    /// use clap::Command;
    /// use crate::arg_buliders::count_arg_builder::CountArgBuilder;
    /// use crate::handlers::build_command_handler::CommandBuilder;
    ///
    /// let mut cmd = Command::new("test_command");
    /// CountArgBuilder::build(&mut cmd);
    ///
    /// // The `--count` argument should be added to the command.
    /// let count_arg = cmd.get_arguments().find(|arg| arg.get_id() == "count");
    /// assert!(count_arg.is_some(), "The 'count' argument was not added.");
    ///
    /// // Verify that a boolean is retrieved from the `--count` argument.
    /// let matches = cmd.clone().try_get_matches_from(vec!["test_command", "--count"]).unwrap();
    /// assert!(matches.get_one::<bool>("count").copied().unwrap_or(false), "The 'count' argument did not return true.");
    /// ```
    fn build(cmd: &mut Command) {
        cmd.arg(
            Arg::new("count")
                .long("count")
                .help("Count all matches")
                .action(clap::ArgAction::SetTrue),
        );
    }
}
