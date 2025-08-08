use crate::handlers::build_command_handler::CommandBuilder;
use clap::{Arg, Command};

pub struct CountBySourceArgBuilder;

impl CommandBuilder for CountBySourceArgBuilder {
    /// Adds the `--count-by-source` argument to the given command.
    ///
    /// The `--count-by-source` argument is a flag that, when provided, sets its value to `true`.
    ///
    /// # Examples
    ///
    /// ```
    /// use clap::Command;
    /// use crate::arg_buliders::count_by_source_arg_builder::CountBySourceArgBuilder;
    /// use crate::handlers::build_command_handler::CommandBuilder;
    ///
    /// let mut cmd = Command::new("test_command");
    /// CountBySourceArgBuilder::build(&mut cmd);
    ///
    /// // The `--count-by-source` argument should be added to the command.
    /// let count_by_source_arg = cmd.get_arguments().find(|arg| arg.get_id() == "count-by-source");
    /// assert!(count_by_source_arg.is_some(), "The 'count-by-source' argument was not added.");
    ///
    /// // Verify that a boolean is retrieved from the `--count-by-source` argument.
    /// let matches = cmd.clone().try_get_matches_from(vec!["test_command", "--count-by-source"]).unwrap();
    /// assert!(matches.get_one::<bool>("count-by-source").copied().unwrap_or(false), "The 'count-by-source' argument did not return true.");
    /// ```
    fn build(cmd: &mut Command) {
        *cmd = cmd.clone().arg(
            Arg::new("count-by-source")
                .long("count-by-source")
                .help("Count matches for each source")
                .action(clap::ArgAction::SetTrue),
        );
    }
}
