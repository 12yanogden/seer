use crate::handlers::build_command_handler::CommandBuilder;
use clap::{Arg, Command};

pub struct FindBetweenArgBuilder;

impl CommandBuilder for FindBetweenArgBuilder {
    /// Adds the `--find_between` argument to the given command.
    ///
    /// The `--find_between` argument requires exactly two string values to be provided.
    ///
    /// # Examples
    ///
    /// ```
    /// use clap::Command;
    /// use crate::arg_buliders::find_between_arg_builder::FindBetweenArgBuilder;
    /// use crate::handlers::build_command_handler::CommandBuilder;
    ///
    /// let mut cmd = Command::new("test_command");
    /// FindBetweenArgBuilder::build(&mut cmd);
    ///
    /// // The `--find_between` argument should be added to the command.
    /// let find_between_arg = cmd.get_arguments().find(|arg| arg.get_id() == "find_between");
    /// assert!(find_between_arg.is_some(), "The 'find_between' argument was not added.");
    ///
    /// // The `--find_between` argument should accept exactly two strings.
    /// let matches = cmd.try_get_matches_from(vec!["test_command", "--find_between", "start", "end"]);
    /// assert!(matches.is_ok(), "The 'find_between' argument did not accept two values.");
    /// let matches = matches.unwrap();
    /// let values: Vec<_> = matches.get_many::<String>("find_between").unwrap().collect();
    /// assert_eq!(values, vec!["start", "end"], "The 'find_between' argument did not capture the correct values.");
    /// ```
    fn build(cmd: &mut Command) {
        cmd.arg(
            Arg::new("find_between")
                .long("find_between")
                .value_names(["START", "END"])
                .num_args(2)
                .help("Find matches between the given start and end strings"),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Command;

    #[test]
    fn find_between_argument_rejects_fewer_than_two_values() {
        let mut cmd = Command::new("test_command");
        FindBetweenArgBuilder::build(&mut cmd);

        // Test with no values
        let matches = cmd
            .clone()
            .try_get_matches_from(vec!["test_command", "--find_between"]);
        assert!(
            matches.is_err(),
            "The 'find_between' argument did not require two values."
        );

        // Test with one value
        let matches = cmd.try_get_matches_from(vec!["test_command", "--find_between", "start"]);
        assert!(
            matches.is_err(),
            "The 'find_between' argument did not require exactly two values."
        );
    }
}
