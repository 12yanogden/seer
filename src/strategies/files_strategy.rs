use crate::dto::dto::DTO;
use crate::strategies::traits::strategy::Strategy;
use crate::strategies::enums::strategy_type::StrategyType;
use crate::read_file;

/// A strategy for processing multiple file inputs.
/// 
/// This strategy reads the contents of files specified by the `--files` command line argument
/// and adds them as sources to the DTO. Each file path is used as the source name.
/// The `--files` argument accepts a comma-separated list of file paths.
/// 
/// # Examples
/// 
/// ```rust
/// use seer::strategies::files_strategy::FilesStrategy;
/// use seer::strategies::traits::strategy::Strategy;
/// use seer::dto::dto::DTO;
/// use seer::arg_buliders::files_arg_builder::FilesArgBuilder;
/// use seer::handlers::build_command_handler::CommandBuilder;
/// use clap::Command;
/// 
/// // Create a DTO
/// let mut dto = DTO::new("seer", "1.0.0", "author", "A search tool");
/// 
/// // Create command with files argument using FilesArgBuilder
/// let mut app = Command::new("test");
/// FilesArgBuilder::build(&mut app);
/// 
/// let matches = app.try_get_matches_from(vec!["test", "--files", "file1.txt,file2.txt"]).unwrap();
/// dto.set_inputs(matches);
/// 
/// // Create and run the strategy
/// let strategy = FilesStrategy::new();
/// strategy.run(&mut dto);
/// 
/// // The file contents should now be added as sources
/// assert_eq!(dto.get_source_count(), 2);
/// ```
pub struct FilesStrategy;

impl FilesStrategy {
    /// Creates a new FilesStrategy instance.
    pub fn new() -> Self {
        Self
    }
}

impl Strategy for FilesStrategy {
    /// Returns the strategy type identifier.
    fn strategy_type(&self) -> StrategyType {
        StrategyType::Files
    }

    /// Executes the files reading strategy.
    /// 
    /// This method:
    /// 1. Attempts to extract the file paths from the `--files` argument in the DTO's inputs
    /// 2. For each file path found, reads the contents of the specified file
    /// 3. Adds each file content as a source to the DTO using the file path as the source name
    /// 
    /// If no `--files` argument is present or a file cannot be read, that file is skipped
    /// and the method continues processing other files, allowing the application to continue.
    fn run(&self, dto: &mut DTO) {
        // Attempt to get the file paths from command line arguments
        if let Some(file_paths) = dto.get_input::<Vec<String>>("files").cloned() {
            // Process each file path
            for file_path in file_paths {
                // Attempt to read the file contents
                match read_file(&file_path) {
                    Ok(file_content) => {
                        // Add the file content as a source to the DTO
                        dto.add_source(&file_path, &file_content);
                    },
                    Err(_) => {
                        // File couldn't be read - silently continue with next file
                        // The SourceStrategyFactory or higher-level error handling
                        // should handle file reading errors appropriately
                    }
                }
            }
        }
        // If no files argument is present, do nothing
        // The SourceStrategyFactory should ensure this strategy is only called
        // when appropriate arguments are present
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arg_buliders::files_arg_builder::FilesArgBuilder;
    use crate::handlers::build_command_handler::CommandBuilder;
    use clap::Command;
    use std::io::Write;
    use tempfile::NamedTempFile;

    lazy_static::lazy_static! {
        static ref CMD: Command = {
            let mut cmd = Command::new("test");
            FilesArgBuilder::build(&mut cmd);
            cmd
        };
        
        static ref BASE_DTO: DTO = {
            DTO::new("seer", "1.0.0", "author", "A search tool")
        };
    }

    #[test]
    fn test_strategy_type() {
        let strategy = FilesStrategy::new();
        assert_eq!(strategy.strategy_type(), StrategyType::Files);
    }

    #[test]
    fn test_run_with_valid_files() {
        // Create temporary files with test content
        let mut temp_file1 = NamedTempFile::new().unwrap();
        writeln!(temp_file1, "Hello from file 1!").unwrap();
        writeln!(temp_file1, "This is the first test file.").unwrap();
        let file_path1 = temp_file1.path().to_str().unwrap();

        let mut temp_file2 = NamedTempFile::new().unwrap();
        writeln!(temp_file2, "Hello from file 2!").unwrap();
        writeln!(temp_file2, "This is the second test file.").unwrap();
        let file_path2 = temp_file2.path().to_str().unwrap();

        // Clone DTO and set inputs
        let mut dto = BASE_DTO.clone();
        let cmd = CMD.clone();
        let files_arg = format!("{},{}", file_path1, file_path2);
        let matches = cmd.try_get_matches_from(vec!["test", "--files", &files_arg]).unwrap();
        dto.set_inputs(matches);

        // Run the strategy
        let strategy = FilesStrategy::new();
        strategy.run(&mut dto);

        // Verify both sources were added
        assert_eq!(dto.get_source_count(), 2);
        let sources = dto.get_sources();
        
        // Check first file
        let source1 = sources.iter().find(|s| s.get_name() == file_path1).unwrap();
        assert!(source1.get_text().contains("Hello from file 1!"));
        assert!(source1.get_text().contains("This is the first test file."));
        
        // Check second file
        let source2 = sources.iter().find(|s| s.get_name() == file_path2).unwrap();
        assert!(source2.get_text().contains("Hello from file 2!"));
        assert!(source2.get_text().contains("This is the second test file."));
    }

    #[test]
    fn test_run_with_single_file() {
        // Create a temporary file with test content
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "Single file content").unwrap();
        let file_path = temp_file.path().to_str().unwrap();

        // Clone DTO and set inputs
        let mut dto = BASE_DTO.clone();
        let cmd = CMD.clone();
        let matches = cmd.try_get_matches_from(vec!["test", "--files", file_path]).unwrap();
        dto.set_inputs(matches);

        // Run the strategy
        let strategy = FilesStrategy::new();
        strategy.run(&mut dto);

        // Verify the source was added
        assert_eq!(dto.get_source_count(), 1);
        let sources = dto.get_sources();
        assert_eq!(sources[0].get_name(), file_path);
        assert!(sources[0].get_text().contains("Single file content"));
    }

    #[test]
    fn test_run_without_files_argument() {
        let mut dto = BASE_DTO.clone();
        let cmd = CMD.clone();
        let matches = cmd.try_get_matches_from(vec!["test"]).unwrap();
        dto.set_inputs(matches);

        let strategy = FilesStrategy::new();
        strategy.run(&mut dto);

        // Should not add any sources when no files argument is present
        assert_eq!(dto.get_source_count(), 0);
    }

    #[test]
    fn test_run_with_mixed_valid_and_invalid_files() {
        // Create one valid temporary file
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "Valid file content").unwrap();
        let valid_file_path = temp_file.path().to_str().unwrap();
        let invalid_file_path = "nonexistent_file.txt";

        let mut dto = BASE_DTO.clone();
        let cmd = CMD.clone();
        let files_arg = format!("{},{}", valid_file_path, invalid_file_path);
        let matches = cmd.try_get_matches_from(vec!["test", "--files", &files_arg]).unwrap();
        dto.set_inputs(matches);

        let strategy = FilesStrategy::new();
        strategy.run(&mut dto);

        // Should add only the valid file as a source
        assert_eq!(dto.get_source_count(), 1);
        let sources = dto.get_sources();
        assert_eq!(sources[0].get_name(), valid_file_path);
        assert!(sources[0].get_text().contains("Valid file content"));
    }

    #[test]
    fn test_run_with_all_nonexistent_files() {
        let mut dto = BASE_DTO.clone();
        let cmd = CMD.clone();
        let matches = cmd.try_get_matches_from(vec!["test", "--files", "file1.txt,file2.txt"]).unwrap();
        dto.set_inputs(matches);

        let strategy = FilesStrategy::new();
        strategy.run(&mut dto);

        // Should not add any sources when all files cannot be read
        assert_eq!(dto.get_source_count(), 0);
    }

    #[test]
    fn test_run_with_empty_file() {
        // Create an empty temporary file
        let temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path().to_str().unwrap();

        // Clone DTO and set inputs
        let mut dto = BASE_DTO.clone();
        let cmd = CMD.clone();
        let matches = cmd.try_get_matches_from(vec!["test", "--files", file_path]).unwrap();
        dto.set_inputs(matches);

        // Run the strategy
        let strategy = FilesStrategy::new();
        strategy.run(&mut dto);

        // Verify the empty file was still added as a source
        assert_eq!(dto.get_source_count(), 1);
        let sources = dto.get_sources();
        assert_eq!(sources[0].get_name(), file_path);
        assert_eq!(sources[0].get_text(), ""); // Empty content
    }
}
