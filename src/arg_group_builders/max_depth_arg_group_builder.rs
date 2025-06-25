use crate::handlers::build_command_handler::CommandBuilder;
use clap::{ArgGroup, Command};

pub struct MaxDepthArgGroupBuilder;

impl CommandBuilder for MaxDepthArgGroupBuilder {
    /// Adds a mutually dependent argument group for `--dir` and `--max-depth`.
    ///
    /// This ensures that the `--max-depth` argument cannot be used without the `--dir` argument.
    ///
    /// # Examples
    ///
    /// ```
    /// use clap::Command;
    /// use crate::arg_group_builders::dir_arg_group_builder::MaxDepthArgGroupBuilder;
    /// use crate::handlers::build_command_handler::CommandBuilder;
    ///
    /// let mut cmd = Command::new("test_command");
    /// MaxDepthArgGroupBuilder::build(&mut cmd);
    ///
    /// // The `--max-depth` argument cannot be used without `--dir`.
    /// let matches = cmd.try_get_matches_from(vec!["test_command", "--max-depth", "3"]);
    /// assert!(matches.is_err(), "The 'max-depth' argument was used without 'dir'.");
    ///
    /// // Both `--dir` and `--max-depth` can be used together.
    /// let matches = cmd.try_get_matches_from(vec!["test_command", "--dir", "/path", "--max-depth", "3"]);
    /// assert!(matches.is_ok(), "The 'dir' and 'max-depth' arguments were not accepted together.");
    /// ```
    fn build(cmd: &mut Command) {
        cmd.group(
            ArgGroup::new("max_depth_arg_group")
                .args(&["dir", "max-depth"])
                .multiple(false)
                .conflicts_with("max-depth"),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Command;

    #[test]
    fn dir_can_be_used_without_max_depth() {
        let mut cmd = Command::new("test_command");
        MaxDepthArgGroupBuilder::build(&mut cmd);
        let matches = cmd.try_get_matches_from(vec!["test_command", "--dir", "/path"]);
        assert!(
            matches.is_ok(),
            "The 'dir' argument was not accepted without 'max-depth'."
        );
    }
}
