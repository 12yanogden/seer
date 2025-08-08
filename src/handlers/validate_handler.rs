use crate::dto::dto::DTO;
use crate::handlers::handler::Handler;

/// Handler for validating DTOs.
///
/// # Example
/// ```
/// use seer::handlers::validate_handler::ValidateHandler;
/// use seer::handlers::handler::Handler;
/// use seer::dto::dto::DTO;
///
/// let mut handler = ValidateHandler;
/// let mut dto = DTO::new("test_command", "1.0", "author_name", "description",);
/// handler.handle(&mut dto);
/// // The DTO is validated. No errors thrown.
/// ```
pub struct ValidateHandler;

impl Handler for ValidateHandler {
    fn handle(&mut self, dto: &mut DTO) {
        verify_a_source_was_given(dto);
        verify_file_sys_paths(dto);
    }
}

/// Checks that at least one of the source options ("dir", "file", "files", or "text")
/// is present in the DTO's inputs.
///
/// # Example
/// ```
/// use seer::dto::dto::DTO;
/// use seer::handlers::validate_handler::verify_a_source_was_given;
/// use seer::arg_buliders::text_arg_builder::TextArgBuilder;
/// use clap::Command;
///
/// let mut cmd = Command::new("test_command");
/// TextArgBuilder::build(&mut cmd);
/// let matches = cmd.get_matches_from(vec!["test_command", "--text", "hello"]);
/// let mut dto = DTO::new("test_command", "1.0", "author_name", "description");
/// dto.set_inputs(matches);
/// verify_a_source_was_given(&dto);
/// ```
fn verify_a_source_was_given(dto: &DTO) {
    let inputs = dto.get_inputs();
    let has_source = inputs.as_ref().map_or(false, |inputs| {
        inputs.contains_id("dir")
            || inputs.contains_id("file")
            || inputs.contains_id("files")
            || inputs.contains_id("text")
    });
    if !has_source {
        panic!("No source provided.");
    }
}

/// Verifies that the values provided to `--dir`, `--file`, and `--files` are valid paths.
///
/// - `--dir` must be an existing directory.
/// - `--file` and each value in `--files` must be existing files.
///
/// # Example
/// ```
/// use seer::dto::dto::DTO;
/// use seer::handlers::validate_handler::verify_file_sys_paths;
/// use seer::arg_buliders::dir_arg_builder::DirArgBuilder;
/// use seer::arg_buliders::file_arg_builder::FileArgBuilder;
/// use seer::arg_buliders::files_arg_builder::FilesArgBuilder;
/// use clap::Command;
/// use std::fs::{File, create_dir_all};
/// use std::env::temp_dir;
///
/// // Setup temp files and directory
/// let tmp = temp_dir();
/// let dir_path = tmp.join("seer_test_dir");
/// let file_path = tmp.join("seer_test_file.txt");
/// let files_path1 = tmp.join("seer_test_file1.txt");
/// let files_path2 = tmp.join("seer_test_file2.txt");
/// create_dir_all(&dir_path).unwrap();
/// File::create(&file_path).unwrap();
/// File::create(&files_path1).unwrap();
/// File::create(&files_path2).unwrap();
///
/// let mut cmd = Command::new("test_command");
/// DirArgBuilder::build(&mut cmd);
/// FileArgBuilder::build(&mut cmd);
/// FilesArgBuilder::build(&mut cmd);
/// let files_arg = format!("{},{}", files_path1.display(), files_path2.display());
/// let matches = cmd.get_matches();
/// let mut dto = DTO::new("test_command", "1.0", "author_name", "description");
/// dto.set_inputs(matches);
/// verify_file_sys_paths(&dto); // Should not panic
/// ```
fn verify_file_sys_paths(dto: &DTO) {
    use std::path::Path;

    let inputs = dto.get_inputs();
    if let Some(inputs) = inputs.as_ref() {
        // Check --dir
        if let Some(dir) = inputs.get_one::<String>("dir") {
            let path = Path::new(dir);
            if !path.is_dir() {
                panic!(
                    "The path provided to --dir does not exist or is not a directory: {}",
                    dir
                );
            }
        }
        // Check --file
        if let Some(file) = inputs.get_one::<String>("file") {
            let path = Path::new(file);
            if !path.is_file() {
                panic!(
                    "The path provided to --file does not exist or is not a file: {}",
                    file
                );
            }
        }
        // Check --files
        if let Some(files) = inputs.get_many::<String>("files") {
            for file in files {
                let path = Path::new(file);
                if !path.is_file() {
                    panic!(
                        "The path provided in --files does not exist or is not a file: {}",
                        file
                    );
                }
            }
        }
    }
}
