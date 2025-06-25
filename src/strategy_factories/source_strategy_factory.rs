use super::{
    dir::DirSourceStrategy, file::FileSourceStrategy, files::FilesSourceStrategy,
    pipe::PipeSourceStrategy, source_strategy::SourceStrategy, text::TextSourceStrategy,
};
use crate::source::source_strategies::source_command_builder::SourceCommandBuilder;
use clap::ArgMatches;

/// Factory for creating source strategies based on command line arguments and pipe content.
pub struct SourceStrategyFactory;

impl SourceStrategyFactory {
    /// Creates a source strategy based on the provided command line arguments and pipe content.
    ///
    /// # Arguments
    ///
    /// * `inputs` - A reference to `ArgMatches` containing the command line arguments.
    /// * `pipe` - An `Option<String>` containing the content of the pipe, if any.
    ///
    /// # Returns
    ///
    /// A boxed `SourceStrategy` instance.
    ///
    /// # Panics
    ///
    /// Panics if no valid source strategy is provided.
    pub fn make(inputs: &ArgMatches, pipe: Option<String>) -> Box<dyn SourceStrategy> {
        if let Some(pipe_content) = pipe {
            return Box::new(PipeSourceStrategy::new(pipe_content));
        } else if inputs.contains_id("dir") {
            return Box::new(DirSourceStrategy::new(inputs));
        } else if inputs.contains_id("file") {
            return Box::new(FileSourceStrategy::new(inputs));
        } else if inputs.contains_id("files") {
            return Box::new(FilesSourceStrategy::new(inputs));
        } else if inputs.contains_id("text") {
            return Box::new(TextSourceStrategy::new(inputs));
        }

        panic!("A source strategy must be provided");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Command;

    fn get_command() -> Command {
        SourceCommandBuilder::build(Command::new("test"))
    }

    #[test]
    fn test_pipe_source_strategy() {
        let cmd = get_command();
        let matches = cmd.try_get_matches_from(vec!["test"]).unwrap();
        let strategy = SourceStrategyFactory::make(&matches, Some("pipe content".to_string()));
        assert_eq!(strategy.strategy_type(), SourceStrategyType::Pipe);
    }

    #[test]
    fn test_dir_source_strategy() {
        let cmd = get_command();
        let matches = cmd
            .try_get_matches_from(vec!["test", "--dir", "path/to/dir"])
            .unwrap();
        let strategy = SourceStrategyFactory::make(&matches, None);
        assert_eq!(strategy.strategy_type(), SourceStrategyType::Dir);
    }

    #[test]
    fn test_file_source_strategy() {
        let cmd = get_command();
        let matches = cmd
            .try_get_matches_from(vec!["test", "--file", "path/to/file"])
            .unwrap();
        let strategy = SourceStrategyFactory::make(&matches, None);
        assert_eq!(strategy.strategy_type(), SourceStrategyType::File);
    }

    #[test]
    fn test_files_source_strategy() {
        let cmd = get_command();
        let matches = cmd
            .try_get_matches_from(vec!["test", "--files", "file1.txt,file2.txt"])
            .unwrap();
        let strategy = SourceStrategyFactory::make(&matches, None);
        assert_eq!(strategy.strategy_type(), SourceStrategyType::Files);
    }

    #[test]
    fn test_text_source_strategy() {
        let cmd = get_command();
        let matches = cmd
            .try_get_matches_from(vec!["test", "--text", "example text"])
            .unwrap();
        let strategy = SourceStrategyFactory::make(&matches, None);
        assert_eq!(strategy.strategy_type(), SourceStrategyType::Text);
    }

    #[test]
    #[should_panic(expected = "A source strategy must be provided")]
    fn test_no_source_strategy() {
        let cmd = get_command();
        let matches = cmd.try_get_matches_from(vec!["test"]).unwrap();
        SourceStrategyFactory::make(&matches, None);
    }
}
