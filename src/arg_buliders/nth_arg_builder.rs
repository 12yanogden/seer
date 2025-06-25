use crate::handlers::build_command_handler::CommandBuilder;
use clap::{Arg, Command};

pub struct NthArgBuilder;

impl CommandBuilder for NthArgBuilder {
    /// Adds the `--nth` argument to the given command.
    ///
    /// The `--nth` argument requires an integer value greater than or equal to 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use clap::Command;
    /// use crate::arg_buliders::nth_arg_builder::NthArgBuilder;
    /// use crate::handlers::build_command_handler::CommandBuilder;
    ///
    /// let mut cmd = Command::new("test_command");
    /// NthArgBuilder::build(&mut cmd);
    ///
    /// // The `--nth` argument should be added to the command.
    /// let nth_arg = cmd.get_arguments().find(|arg| arg.get_id() == "nth");
    /// assert!(nth_arg.is_some(), "The 'nth' argument was not added.");
    ///
    /// // The `--nth` argument should accept a valid integer.
    /// let matches = cmd.try_get_matches_from(vec!["test_command", "--nth", "3"]);
    /// assert!(matches.is_ok(), "The 'nth' argument did not accept a valid value.");
    /// let matches = matches.unwrap();
    /// assert_eq!(
    ///     matches.get_one::<String>("nth").unwrap(),
    ///     "3",
    ///     "The 'nth' argument did not capture the correct value."
    /// );
    /// ```
    fn build(cmd: &mut Command) {
        cmd.arg(
            Arg::new("nth")
                .long("nth")
                .value_name("NTH")
                .value_parser(clap::value_parser!(u64).range(0..))
                .help("Find/edit only the nth match"),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Command;

    #[test]
    fn nth_argument_throws_error_without_a_value_given() {
        let mut cmd = Command::new("test_command");
        NthArgBuilder::build(&mut cmd);
        let matches = cmd.try_get_matches_from(vec!["test_command", "--nth"]);
        assert!(
            matches.is_err(),
            "The 'nth' argument did not require a value."
        );
    }

    #[test]
    fn nth_argument_requires_valid_integer() {
        let mut cmd = Command::new("test_command");
        NthArgBuilder::build(&mut cmd);
        let matches = cmd.try_get_matches_from(vec!["test_command", "--nth", "-1"]);
        assert!(
            matches.is_err(),
            "The 'nth' argument accepted an invalid value."
        );
    }
}
