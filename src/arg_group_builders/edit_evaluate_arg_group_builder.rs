use crate::handlers::build_command_handler::CommandBuilder;
use clap::{ArgGroup, Command};

/// Now handles Transform phase argument restrictions
pub struct EditEvaluateArgGroupBuilder;

impl CommandBuilder for EditEvaluateArgGroupBuilder {
    /// Adds argument group restrictions for Transform phase operations.
    /// Based on Operation Dependency Tree: Edit operations (append, prepend, replace-with) 
    /// are incompatible with Evaluation operations (count, count-by-source).
    ///
    /// # Examples
    ///
    /// ```
    /// use clap::Command;
    /// use crate::arg_group_builders::edit_evaluate_arg_group_builder::EditEvaluateArgGroupBuilder;
    /// use crate::handlers::build_command_handler::CommandBuilder;
    ///
    /// let mut cmd = Command::new("test_command");
    /// EditEvaluateArgGroupBuilder::build(&mut cmd);
    ///
    /// // Transform phase: Edit operations cannot be used with Evaluation operations
    /// let matches = cmd.try_get_matches_from(vec!["test_command", "--replace-with", "value", "--count"]);
    /// assert!(matches.is_err(), "Edit and Evaluation Transform operations are incompatible.");
    /// ```
    fn build(cmd: &mut Command) {
        *cmd = cmd.clone().group(
            ArgGroup::new("transform_edit_group")
                .args(&["append", "prepend", "replace-with"]) // Edit Transform operations
                .conflicts_with_all(["count", "count-by-source"]) // Evaluation Transform operations
                .multiple(false),
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
            EditEvaluateArgGroupBuilder::build(&mut cmd);
            cmd
        };
    }

    #[test]
    fn append_cannot_be_used_with_count() {
        let cmd = CMD.clone();
        let matches =
            cmd.try_get_matches_from(vec!["test_command", "--append", "value", "--count"]);
        assert!(
            matches.is_err(),
            "Transform phase: Edit and Evaluation operations are incompatible."
        );
    }

    #[test]
    fn prepend_cannot_be_used_with_count() {
        let cmd = CMD.clone();
        let matches =
            cmd.try_get_matches_from(vec!["test_command", "--prepend", "value", "--count"]);
        assert!(
            matches.is_err(),
            "Transform phase: Edit and Evaluation operations are incompatible."
        );
    }
}
