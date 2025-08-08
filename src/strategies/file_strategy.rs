use crate::dto::dto::DTO;
use crate::strategies::traits::strategy::Strategy;
use crate::strategies::enums::strategy_type::StrategyType;
use crate::read_file;

/// A strategy for processing a single file input.
/// 
/// This strategy reads the contents of a file specified by the `--file` command line argument
/// and adds it as a source to the DTO. The file path is used as the source name.
/// 
/// # Examples
/// 
/// ```rust
/// use seer::strategies::file_strategy::FileStrategy;
/// use seer::strategies::traits::strategy::Strategy;
/// use seer::dto::dto::DTO;
/// use seer::arg_buliders::file_arg_builder::FileArgBuilder;
/// use seer::handlers::build_command_handler::CommandBuilder;
/// use clap::Command;
/// 
/// // Create a DTO
/// let mut dto = DTO::new("seer", "1.0.0", "author", "A search tool");
/// 
/// // Create command with file argument using FileArgBuilder
/// let mut app = Command::new("test");
/// FileArgBuilder::build(&mut app);
/// 
/// let matches = app.try_get_matches_from(vec!["test", "--file", "example.txt"]).unwrap();
/// dto.set_inputs(matches);
/// 
/// // Create and run the strategy
/// let strategy = FileStrategy::new();
/// strategy.run(&mut dto);
/// 
/// // The file content should now be added as a source
/// assert_eq!(dto.get_source_count(), 1);
/// ```
pub struct FileStrategy;

impl FileStrategy {
    /// Creates a new FileStrategy instance.
    pub fn new() -> Self {
        Self
    }
}

impl Strategy for FileStrategy {
    /// Returns the strategy type identifier.
    fn strategy_type(&self) -> StrategyType {
        StrategyType::File
    }

    /// Executes the file reading strategy.
    /// 
    /// This method:
    /// 1. Attempts to extract the file path from the `--file` argument in the DTO's inputs
    /// 2. If a file path is found, reads the contents of the specified file
    /// 3. Adds the file content as a source to the DTO using the file path as the source name
    /// 
    /// If no `--file` argument is present or the file cannot be read, the method returns
    /// without adding a source, allowing the application to continue.
    fn run(&self, dto: &mut DTO) {
        // Attempt to get the file path from command line arguments
        if let Some(file_path) = dto.get_input::<String>("file").cloned() {
            // Attempt to read the file contents
            match read_file(&file_path) {
                Ok(file_content) => {
                    // Add the file content as a source to the DTO
                    dto.add_source(&file_path, &file_content);
                },
                Err(_) => {
                    // File couldn't be read - silently continue
                    // The SourceStrategyFactory or higher-level error handling
                    // should handle file reading errors appropriately
                }
            }
        }
        // If no file argument is present, do nothing
        // The SourceStrategyFactory should ensure this strategy is only called
        // when appropriate arguments are present
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arg_buliders::file_arg_builder::FileArgBuilder;
    use crate::handlers::build_command_handler::CommandBuilder;
    use clap::Command;
    use std::io::Write;
    use tempfile::NamedTempFile;

    lazy_static::lazy_static! {
        static ref CMD: Command = {
            let mut cmd = Command::new("test");
            FileArgBuilder::build(&mut cmd);
            cmd
        };
        
        static ref BASE_DTO: DTO = {
            DTO::new("seer", "1.0.0", "author", "A search tool")
        };
    }

    #[test]
    fn test_strategy_type() {
        let strategy = FileStrategy::new();
        assert_eq!(strategy.strategy_type(), StrategyType::File);
    }

    #[test]
    fn test_run_with_valid_file() {
        // Create a temporary file with test content
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "Hello, World!").unwrap();
        writeln!(temp_file, "This is a test file.").unwrap();
        let file_path = temp_file.path().to_str().unwrap();

        // Clone DTO and set inputs
        let mut dto = BASE_DTO.clone();
        let cmd = CMD.clone();
        let matches = cmd.try_get_matches_from(vec!["test", "--file", file_path]).unwrap();
        dto.set_inputs(matches);

        // Run the strategy
        let strategy = FileStrategy::new();
        strategy.run(&mut dto);

        // Verify the source was added
        assert_eq!(dto.get_source_count(), 1);
        let sources = dto.get_sources();
        assert_eq!(sources[0].get_name(), file_path);
        assert!(sources[0].get_text().contains("Hello, World!"));
        assert!(sources[0].get_text().contains("This is a test file."));
    }

    #[test]
    fn test_run_without_file_argument() {
        let mut dto = BASE_DTO.clone();
        let cmd = CMD.clone();
        let matches = cmd.try_get_matches_from(vec!["test"]).unwrap();
        dto.set_inputs(matches);

        let strategy = FileStrategy::new();
        strategy.run(&mut dto);

        // Should not add any sources when no file argument is present
        assert_eq!(dto.get_source_count(), 0);
    }

    #[test]
    fn test_run_with_nonexistent_file() {
        let mut dto = BASE_DTO.clone();
        let cmd = CMD.clone();
        let matches = cmd.try_get_matches_from(vec!["test", "--file", "nonexistent_file.txt"]).unwrap();
        dto.set_inputs(matches);

        let strategy = FileStrategy::new();
        strategy.run(&mut dto);

        // Should not add any sources when file cannot be read
        assert_eq!(dto.get_source_count(), 0);
    }
}
