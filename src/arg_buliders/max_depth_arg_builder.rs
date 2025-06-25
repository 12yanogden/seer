use crate::handlers::build_command_handler::CommandBuilder;
use clap::{Arg, Command};

pub struct MaxDepthArgBuilder;

impl CommandBuilder for MaxDepthArgBuilder {
    /// Adds the `--max-depth` argument to the given command.
    ///
    /// The `--max-depth` argument requires an integer value greater than or equal to 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use clap::Command;
    /// use crate::arg_buliders::max_depth_arg_builder::MaxDepthArgBuilder;
    /// use crate::handlers::build_command_handler::CommandBuilder;
    ///
    /// let mut cmd = Command::new("test_command");
    /// MaxDepthArgBuilder::build(&mut cmd);
    ///
    /// // The `--max-depth` argument should be added to the command.
    /// let max_depth_arg = cmd.get_arguments().find(|arg| arg.get_id() == "max-depth");
    /// assert!(max_depth_arg.is_some(), "The 'max-depth' argument was not added.");
    ///
    /// // The `--max-depth` argument should accept a valid integer.
    /// let matches = cmd.try_get_matches_from(vec!["test_command", "--max-depth", "3"]);
    /// assert!(matches.is_ok(), "The 'max-depth' argument did not accept a valid value.");
    /// let matches = matches.unwrap();
    /// assert_eq!(
    ///     matches.get_one::<String>("max-depth").unwrap(),
    ///     "3",
    ///     "The 'max-depth' argument did not capture the correct value."
    /// );
    /// ```
    fn build(cmd: &mut Command) {
        cmd.arg(
            Arg::new("max-depth")
                .long("max-depth")
                .value_name("MAX_DEPTH")
                .value_parser(clap::value_parser!(u64).range(0..))
                .help("Limit the depth of recursion"),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Command;

    #[test]
    fn max_depth_argument_throws_error_without_a_value_given() {
        let mut cmd = Command::new("test_command");
        MaxDepthArgBuilder::build(&mut cmd);
        let matches = cmd.try_get_matches_from(vec!["test_command", "--max-depth"]);
        assert!(
            matches.is_err(),
            "The 'max-depth' argument did not require a value."
        );
    }

    #[test]
    fn max_depth_argument_requires_valid_integer() {
        let mut cmd = Command::new("test_command");
        MaxDepthArgBuilder::build(&mut cmd);
        let matches = cmd.try_get_matches_from(vec!["test_command", "--max-depth", "-1"]);
        assert!(
            matches.is_err(),
            "The 'max-depth' argument accepted an invalid value."
        );
    }
}
