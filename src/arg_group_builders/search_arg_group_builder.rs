use crate::handlers::build_command_handler::CommandBuilder;
use clap::{ArgGroup, Command};

pub struct SearchArgGroupBuilder;

impl CommandBuilder for SearchArgGroupBuilder {
    /// Adds a mutually exclusive argument group for Search arguments.
    ///
    /// # Examples
    ///
    /// ```
    /// use clap::Command;
    /// use crate::arg_group_builders::search_arg_group_builder::SearchArgGroupBuilder;
    /// use crate::handlers::build_command_handler::CommandBuilder;
    ///
    /// let mut cmd = Command::new("test_command");
    /// SearchArgGroupBuilder::build(&mut cmd);
    ///
    /// // At least one argument is required.
    /// let matches = cmd.try_get_matches_from(vec!["test_command"]);
    /// assert!(matches.is_err(), "No argument was provided, but at least one is required.");
    ///
    /// // Search arguments like `find_between` cannot be used with other search arguments like `find_regex`.
    /// let matches = cmd.try_get_matches_from(vec!["test_command", "--find_between", "--find_regex"]);
    /// assert!(matches.is_err(), "The 'find_between' argument was used with 'find_regex'.");
    /// ```
    fn build(cmd: &mut Command) {
        cmd.group(
            ArgGroup::new("search_arg_group")
                .args(&["find_between", "find_regex", "find_string"]) // Search arguments
                .multiple(false) // Ensure zero or one argument is allowed
                .required(true), // Ensure at least one argument is required
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
            SearchArgGroupBuilder::build(&mut cmd);
            cmd
        };
    }

    #[test]
    fn find_between_cannot_be_used_with_find_regex() {
        let cmd = CMD.clone();
        let matches =
            cmd.try_get_matches_from(vec!["test_command", "--find_between", "--find_regex"]);
        assert!(
            matches.is_err(),
            "The 'find_between' argument was used with 'find_regex'."
        );
    }

    #[test]
    fn find_regex_cannot_be_used_with_find_string() {
        let cmd = CMD.clone();
        let matches =
            cmd.try_get_matches_from(vec!["test_command", "--find_regex", "--find_string"]);
        assert!(
            matches.is_err(),
            "The 'find_regex' argument was used with 'find_string'."
        );
    }
}
