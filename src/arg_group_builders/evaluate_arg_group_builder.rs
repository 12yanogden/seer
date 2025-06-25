use crate::handlers::build_command_handler::CommandBuilder;
use clap::{ArgGroup, Command};

pub struct EvaluateArgGroupBuilder;

impl CommandBuilder for EvaluateArgGroupBuilder {
    /// Adds a mutually exclusive argument group for Evaluate arguments.
    ///
    /// # Examples
    ///
    /// ```
    /// use clap::Command;
    /// use crate::arg_group_builders::evaluate_arg_group_builder::EvaluateArgGroupBuilder;
    /// use crate::handlers::build_command_handler::CommandBuilder;
    ///
    /// let mut cmd = Command::new("test_command");
    /// EvaluateArgGroupBuilder::build(&mut cmd);
    ///
    /// // Evaluate arguments like `count` cannot be used with other evaluate arguments like `count_by_source`.
    /// let matches = cmd.try_get_matches_from(vec!["test_command", "--count", "--count_by_source"]);
    /// assert!(matches.is_err(), "The 'count' argument was used with 'count_by_source'.");
    /// ```
    fn build(cmd: &mut Command) {
        cmd.group(
            ArgGroup::new("evaluate_arg_group")
                .args(&["count", "count_by_source"]) // Evaluate arguments
                .multiple(false), // Ensure zero or one argument is allowed
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Command;

    lazy_static::lazy_static! {
        static ref CMD: Command = {
            let mut cmd = Command::new("test_command");
            EvaluateArgGroupBuilder::build(&mut cmd);
            cmd
        };
    }

    #[test]
    fn count_cannot_be_used_with_count_by_source() {
        let cmd = CMD.clone();
        let matches =
            cmd.try_get_matches_from(vec!["test_command", "--count", "--count_by_source"]);
        assert!(
            matches.is_err(),
            "The 'count' argument was used with 'count_by_source'."
        );
    }
}
