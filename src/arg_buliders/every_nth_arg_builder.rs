use crate::handlers::build_command_handler::CommandBuilder;
use clap::{Arg, Command};

pub struct EveryNthArgBuilder;

impl CommandBuilder for EveryNthArgBuilder {
    /// Adds the `--every_nth` argument to the given command.
    ///
    /// The `--every_nth` argument requires an integer value greater than or equal to 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use clap::Command;
    /// use crate::arg_buliders::every_nth_arg_builder::EveryNthArgBuilder;
    /// use crate::handlers::build_command_handler::CommandBuilder;
    ///
    /// let mut cmd = Command::new("test_command");
    /// EveryNthArgBuilder::build(&mut cmd);
    ///
    /// // The `--every_nth` argument should be added to the command.
    /// let every_nth_arg = cmd.get_arguments().find(|arg| arg.get_id() == "every_nth");
    /// assert!(every_nth_arg.is_some(), "The 'every_nth' argument was not added.");
    ///
    /// // The `--every_nth` argument should accept a valid integer.
    /// let matches = cmd.try_get_matches_from(vec!["test_command", "--every_nth", "5"]);
    /// assert!(matches.is_ok(), "The 'every_nth' argument did not accept a valid value.");
    /// let matches = matches.unwrap();
    /// assert_eq!(
    ///     matches.get_one::<String>("every_nth").unwrap(),
    ///     "5",
    ///     "The 'every_nth' argument did not capture the correct value."
    /// );
    /// ```
    fn build(cmd: &mut Command) {
        cmd.arg(
            Arg::new("every_nth")
                .long("every_nth")
                .value_name("N")
                .value_parser(clap::value_parser!(u64).range(0..))
                .help("Process every nth match"),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Command;

    #[test]
    fn every_nth_argument_throws_error_without_a_value_given() {
        let mut cmd = Command::new("test_command");
        EveryNthArgBuilder::build(&mut cmd);
        let matches = cmd.try_get_matches_from(vec!["test_command", "--every_nth"]);
        assert!(
            matches.is_err(),
            "The 'every_nth' argument did not require a value."
        );
    }

    #[test]
    fn every_nth_argument_requires_valid_integer() {
        let mut cmd = Command::new("test_command");
        EveryNthArgBuilder::build(&mut cmd);
        let matches = cmd.try_get_matches_from(vec!["test_command", "--every_nth", "-1"]);
        assert!(
            matches.is_err(),
            "The 'every_nth' argument accepted an invalid value."
        );
    }
}
