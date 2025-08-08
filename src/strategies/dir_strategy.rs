use crate::dto::dto::DTO;
use crate::strategies::traits::strategy::Strategy;
use crate::strategies::enums::strategy_type::StrategyType;
use crate::read_file;
use std::path::Path;
use std::fs;

/// A strategy for processing directory input recursively.
/// 
/// This strategy reads a directory path specified by the `--dir` command line argument
/// and recursively processes all text files within it, adding each as a source to the DTO.
/// Each file path is used as the source name.
/// 
/// # Examples
/// 
/// ```rust
/// use seer::strategies::dir_strategy::DirStrategy;
/// use seer::strategies::traits::strategy::Strategy;
/// use seer::dto::dto::DTO;
/// use seer::arg_buliders::dir_arg_builder::DirArgBuilder;
/// use seer::handlers::build_command_handler::CommandBuilder;
/// use clap::Command;
/// 
/// // Create a DTO
/// let mut dto = DTO::new("seer", "1.0.0", "author", "A search tool");
/// 
/// // Create command with dir argument using DirArgBuilder
/// let mut app = Command::new("test");
/// DirArgBuilder::build(&mut app);
/// 
/// let matches = app.try_get_matches_from(vec!["test", "--dir", "./src"]).unwrap();
/// dto.set_inputs(matches);
/// 
/// // Create and run the strategy
/// let strategy = DirStrategy::new();
/// strategy.run(&mut dto);
/// 
/// // All text files in the directory should be added as sources
/// assert!(dto.get_source_count() > 0);
/// ```
pub struct DirStrategy;

impl DirStrategy {
    /// Creates a new DirStrategy instance.
    pub fn new() -> Self {
        Self
    }

    /// Recursively processes a directory and adds all readable files as sources.
    fn process_directory_recursive(dir: &Path, dto: &mut DTO) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    
                    if path.is_dir() {
                        // Recursively process subdirectories
                        Self::process_directory_recursive(&path, dto);
                    } else if path.is_file() {
                        // Process file if it's readable text
                        if let Some(path_str) = path.to_str() {
                            if Self::is_text_file(&path) {
                                if let Ok(content) = read_file(path_str) {
                                    dto.add_source(path_str, &content);
                                }
                                // Skip files that can't be read
                            }
                        }
                    }
                }
            }
        }
    }

    /// Determines if a file is likely a text file based on its extension.
    fn is_text_file(path: &Path) -> bool {
        if let Some(extension) = path.extension() {
            if let Some(ext_str) = extension.to_str() {
                let text_extensions = [
                    "txt", "md", "rs", "py", "js", "ts", "html", "css", "json", "xml", "yml", "yaml",
                    "toml", "ini", "cfg", "conf", "log", "sh", "bash", "zsh", "fish", "ps1", "bat",
                    "c", "cpp", "cc", "cxx", "h", "hpp", "java", "kt", "go", "rb", "php", "pl",
                    "lua", "r", "sql", "csv", "tsv", "dockerfile", "makefile", "cmake", "gradle"
                ];
                return text_extensions.contains(&ext_str.to_lowercase().as_str());
            }
        }
        
        // Also check for common files without extensions
        if let Some(file_name) = path.file_name() {
            if let Some(name_str) = file_name.to_str() {
                let common_text_files = [
                    "README", "LICENSE", "CHANGELOG", "Dockerfile", "Makefile", 
                    "build.gradle", ".gitignore", ".dockerignore"
                ];
                return common_text_files.contains(&name_str);
            }
        }
        
        false
    }
}

impl Strategy for DirStrategy {
    /// Returns the strategy type identifier.
    fn strategy_type(&self) -> StrategyType {
        StrategyType::Dir
    }

    /// Executes the directory processing strategy.
    /// 
    /// This method:
    /// 1. Extracts the directory path from the `--dir` argument
    /// 2. Validates that the path exists and is a directory
    /// 3. Recursively processes all text files in the directory
    /// 4. Adds each file's content as a source to the DTO using the file path as the source name
    /// 
    /// Files that cannot be read are silently skipped, allowing the strategy
    /// to continue processing other files.
    fn run(&self, dto: &mut DTO) {
        // Attempt to get the directory path from command line arguments
        if let Some(dir_path) = dto.get_input::<String>("dir").cloned() {
            let path = Path::new(&dir_path);
            
            // Validate path exists and is a directory
            if path.exists() && path.is_dir() {
                Self::process_directory_recursive(path, dto);
            }
            // If path doesn't exist or isn't a directory, silently continue
        }
        // If no dir argument is present, do nothing
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arg_buliders::dir_arg_builder::DirArgBuilder;
    use crate::handlers::build_command_handler::CommandBuilder;
    use clap::Command;
    use std::io::Write;
    use tempfile::TempDir;
    use std::fs::File;

    #[test]
    fn test_strategy_type() {
        let strategy = DirStrategy::new();
        assert_eq!(strategy.strategy_type(), StrategyType::Dir);
    }

    #[test]
    fn test_run_with_directory_containing_text_files() {
        // Create a temporary directory with test files
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();

        // Create a Rust file
        let rs_file_path = dir_path.join("test.rs");
        let mut rs_file = File::create(&rs_file_path).unwrap();
        writeln!(rs_file, "fn main() {{ println!(\"Hello, world!\"); }}").unwrap();

        // Create a text file
        let txt_file_path = dir_path.join("test.txt");
        let mut txt_file = File::create(&txt_file_path).unwrap();
        writeln!(txt_file, "This is a text file.").unwrap();

        // Create a binary file that should be ignored
        let _bin_file_path = dir_path.join("test.bin");
        let _bin_file = File::create(&_bin_file_path).unwrap();

        // Create DTO and command
        let mut dto = DTO::new("seer", "1.0.0", "author", "A search tool");
        let mut cmd = Command::new("test_command");
        DirArgBuilder::build(&mut cmd);
        
        let matches = cmd.try_get_matches_from(vec!["test", "--dir", dir_path.to_str().unwrap()]).unwrap();
        dto.set_inputs(matches);

        // Run the strategy
        let strategy = DirStrategy::new();
        strategy.run(&mut dto);

        // Verify sources were added (should be 2: .rs and .txt files)
        assert_eq!(dto.get_source_count(), 2);
        let sources = dto.get_sources();
        
        // Check that both text files were processed
        let rs_source = sources.iter().find(|s| s.get_name().ends_with("test.rs")).unwrap();
        assert!(rs_source.get_text().contains("Hello, world!"));
        
        let txt_source = sources.iter().find(|s| s.get_name().ends_with("test.txt")).unwrap();
        assert!(txt_source.get_text().contains("This is a text file."));
    }
}
