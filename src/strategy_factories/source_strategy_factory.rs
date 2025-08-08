use crate::dto::dto::DTO;
use clap::ArgMatches;
use std::io::{self, Read, IsTerminal};
use crate::read_file;
use std::path::Path;
use std::fs;

/// Factory for processing multiple source inputs and adding them to the DTO.
/// Handles pipe input, file input, directory input, files input, and text input.
pub struct SourceStrategyFactory;

impl SourceStrategyFactory {
    /// Processes all available sources and adds them to the DTO.
    ///
    /// # Arguments
    ///
    /// * `inputs` - A reference to `ArgMatches` containing the command line arguments.
    /// * `dto` - A mutable reference to the DTO to add sources to.
    ///
    /// # Returns
    ///
    /// Result indicating success or failure of processing sources.
    pub fn process_sources(inputs: &ArgMatches, dto: &mut DTO) -> Result<(), Box<dyn std::error::Error>> {
        let mut sources_added = false;

        // Check for pipe input first
        if let Some(pipe_content) = Self::read_pipe() {
            dto.add_source("pipe", &pipe_content);
            sources_added = true;
        }

        // Process text input
        if let Some(text) = inputs.get_one::<String>("text") {
            dto.add_source("text", text);
            sources_added = true;
        }

        // Process file input
        if let Some(file_path) = inputs.get_one::<String>("file") {
            match read_file(file_path) {
                Ok(content) => {
                    dto.add_source(file_path, &content);
                    sources_added = true;
                },
                Err(e) => return Err(Box::new(e)),
            }
        }

        // Process files input (comma-separated list)
        if let Some(files_str) = inputs.get_one::<String>("files") {
            for file_path in files_str.split(',').map(|s| s.trim()) {
                if !file_path.is_empty() {
                    match read_file(file_path) {
                        Ok(content) => {
                            dto.add_source(file_path, &content);
                            sources_added = true;
                        },
                        Err(e) => return Err(Box::new(e)),
                    }
                }
            }
        }

        // Process directory input
        if let Some(dir_path) = inputs.get_one::<String>("dir") {
            Self::process_directory(dir_path, dto)?;
            sources_added = true;
        }

        if !sources_added {
            return Err("No source input provided".into());
        }

        Ok(())
    }

    /// Reads piped input from stdin.
    ///
    /// # Returns
    ///
    /// An `Option<String>` containing the piped input if available.
    fn read_pipe() -> Option<String> {
        // Only attempt to read from stdin if it's not a terminal (i.e., it's piped)
        if !io::stdin().is_terminal() {
            let mut pipe = String::new();
            if let Ok(_) = io::stdin().read_to_string(&mut pipe) {
                if !pipe.is_empty() {
                    return Some(pipe);
                }
            }
        }
        None
    }

    /// Recursively processes a directory and adds all readable files as sources.
    ///
    /// # Arguments
    ///
    /// * `dir_path` - The path to the directory to process.
    /// * `dto` - A mutable reference to the DTO to add sources to.
    ///
    /// # Returns
    ///
    /// Result indicating success or failure of processing the directory.
    fn process_directory(dir_path: &str, dto: &mut DTO) -> Result<(), Box<dyn std::error::Error>> {
        let path = Path::new(dir_path);
        if !path.exists() {
            return Err(format!("Directory does not exist: {}", dir_path).into());
        }
        if !path.is_dir() {
            return Err(format!("Path is not a directory: {}", dir_path).into());
        }

        Self::process_directory_recursive(path, dto)?;
        Ok(())
    }

    /// Helper function for recursive directory processing.
    fn process_directory_recursive(dir: &Path, dto: &mut DTO) -> Result<(), Box<dyn std::error::Error>> {
        let entries = fs::read_dir(dir)?;
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                // Recursively process subdirectories
                Self::process_directory_recursive(&path, dto)?;
            } else if path.is_file() {
                // Process file if it's readable text
                if let Some(path_str) = path.to_str() {
                    if Self::is_text_file(&path) {
                        match read_file(path_str) {
                            Ok(content) => {
                                dto.add_source(path_str, &content);
                            },
                            Err(_) => {
                                // Skip files that can't be read
                                continue;
                            }
                        }
                    }
                }
            }
        }
        
        Ok(())
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
                    "CMakeLists.txt", "build.gradle", ".gitignore", ".dockerignore"
                ];
                return common_text_files.contains(&name_str);
            }
        }
        
        false
    }
}
