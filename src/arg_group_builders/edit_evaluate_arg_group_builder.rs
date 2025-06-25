use crate::handlers::build_command_handler::CommandBuilder;
use clap::{ArgGroup, Command};

pub struct EditEvaluateArgGroupBuilder;

impl CommandBuilder for EditEvaluateArgGroupBuilder {
    /// Adds a mutually exclusive argument group for Edit and Evaluate arguments.
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
    /// // Edit arguments like `replace_with` cannot be used with evaluate arguments like `--count`.
    /// let matches = cmd.try_get_matches_from(vec!["test_command", "--replace_with", "value", "--count"]);
    /// assert!(matches.is_err(), "The 'count' argument was used with 'replace_with'.");
    /// ```
    fn build(cmd: &mut Command) {
        cmd.group(
            ArgGroup::new("edit_evaluate_arg_group")
                .args(&["append", "prepend", "replace_with"]) // Edit arguments
                .conflicts_with("count, count_by_source") // Evaluate arguments
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
            "The 'append' argument was used with 'count'."
        );
    }

    #[test]
    fn prepend_cannot_be_used_with_count() {
        let cmd = CMD.clone();
        let matches =
            cmd.try_get_matches_from(vec!["test_command", "--prepend", "value", "--count"]);
        assert!(
            matches.is_err(),
            "The 'prepend' argument was used with 'count'."
        );
    }
}
