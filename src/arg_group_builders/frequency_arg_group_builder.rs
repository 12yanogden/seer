use crate::handlers::build_command_handler::CommandBuilder;
use clap::{ArgGroup, Command};

pub struct FrequencyArgGroupBuilder;

impl CommandBuilder for FrequencyArgGroupBuilder {
    /// Adds a mutually exclusive argument group for Frequency arguments.
    ///
    /// # Examples
    ///
    /// ```
    /// use clap::Command;
    /// use crate::arg_group_builders::frequency_arg_group_builder::FrequencyArgGroupBuilder;
    /// use crate::handlers::build_command_handler::CommandBuilder;
    ///
    /// let mut cmd = Command::new("test_command");
    /// FrequencyArgGroupBuilder::build(&mut cmd);
    ///
    /// // Frequency arguments like `all` cannot be used with other frequency arguments like `every_nth`.
    /// let matches = cmd.try_get_matches_from(vec!["test_command", "--all", "--every_nth"]);
    /// assert!(matches.is_err(), "The 'all' argument was used with 'every_nth'.");
    /// ```
    fn build(cmd: &mut Command) {
        cmd.group(
            ArgGroup::new("frequency_arg_group")
                .args(&["all", "every_nth", "nth"])
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
            FrequencyArgGroupBuilder::build(&mut cmd);
            cmd
        };
    }

    #[test]
    fn all_cannot_be_used_with_every_nth() {
        let cmd = CMD.clone();
        let matches = cmd.try_get_matches_from(vec!["test_command", "--all", "--every_nth"]);
        assert!(
            matches.is_err(),
            "The 'all' argument was used with 'every_nth'."
        );
    }

    #[test]
    fn every_nth_cannot_be_used_with_nth() {
        let cmd = CMD.clone();
        let matches = cmd.try_get_matches_from(vec!["test_command", "--every_nth", "--nth"]);
        assert!(
            matches.is_err(),
            "The 'every_nth' argument was used with 'nth'."
        );
    }
}
