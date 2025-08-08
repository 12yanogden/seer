use crate::handlers::build_command_handler::CommandBuilder;
use clap::{ArgGroup, Command};

/// Handles mutual exclusivity within Transform phase Evaluation operations
pub struct EvaluateArgGroupBuilder;

impl CommandBuilder for EvaluateArgGroupBuilder {
    /// Adds a mutually exclusive argument group for Transform phase Evaluation operations.
    /// Based on Operation Dependency Tree: count and count-by-source are mutually exclusive.
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
    /// // Transform phase: Evaluation operations are mutually exclusive
    /// let matches = cmd.try_get_matches_from(vec!["test_command", "--count", "--count-by-source"]);
    /// assert!(matches.is_err(), "Transform phase Evaluation operations are mutually exclusive.");
    /// ```
    fn build(cmd: &mut Command) {
        *cmd = cmd.clone().group(
            ArgGroup::new("transform_evaluation_group")
                .args(&["count", "count-by-source"]) // Transform phase Evaluation operations
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
            cmd.try_get_matches_from(vec!["test_command", "--count", "--count-by-source"]);
        assert!(
            matches.is_err(),
            "Transform phase: Evaluation operations are mutually exclusive."
        );
    }
}
