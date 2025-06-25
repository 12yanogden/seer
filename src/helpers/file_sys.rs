use std::fs;
use std::io;
use std::path::PathBuf;

/// Returns a vector of file paths from the specified directory.
///
/// # Arguments
///
/// * `dir` - A string slice that holds the path of the directory
///
/// # Example
///
/// ```
/// use std::fs::File;
/// use tempfile::tempdir;
/// use seer::get_file_paths_from_dir;
///
/// let dir = tempdir().unwrap();
/// let dir_path = dir.path();
///
/// let file1 = dir_path.join("file1.txt");
/// let file2 = dir_path.join("file2.txt");
///
/// File::create(&file1).unwrap();
/// File::create(&file2).unwrap();
///
/// let file_paths = get_file_paths_from_dir(dir_path.to_str().unwrap()).unwrap();
/// assert!(file_paths.contains(&file1));
/// assert!(file_paths.contains(&file2));
/// ```
///
/// # Errors
///
/// This function will return an error if the directory does not exist or if it is not a directory.
pub fn get_file_paths_from_dir(dir: &str) -> io::Result<Vec<PathBuf>> {
    let mut file_paths = Vec::new();
    for entry in fs::read_dir(dir).map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("Failed to read directory: '{}'. {}", dir, e),
        )
    })? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            file_paths.push(path);
        }
    }
    Ok(file_paths)
}

#[cfg(test)]
mod get_file_paths_from_dir_tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn test_can_return_file_paths() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path();

        let file1 = dir_path.join("file1.txt");
        let file2 = dir_path.join("file2.txt");

        fs::File::create(&file1).unwrap();
        fs::File::create(&file2).unwrap();

        let file_paths = get_file_paths_from_dir(dir_path.to_str().unwrap()).unwrap();
        assert_eq!(file_paths.len(), 2);
        assert!(file_paths.contains(&file1));
        assert!(file_paths.contains(&file2));
    }

    #[test]
    fn test_get_file_paths_from_invalid_dir() {
        let result = get_file_paths_from_dir("invalid_dir");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Failed to read directory: 'invalid_dir'. No such file or directory (os error 2)"
        );
    }
}

/// Reads the contents of a file given its path.
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the path of the file.
///
/// # Returns
///
/// A `String` containing the contents of the file.
///
/// # Errors
///
/// This function will return an error if the file path is invalid or the file does not exist.
///
/// # Examples
///
/// ```
/// use seer::read_file;
/// use tempfile::NamedTempFile;
/// use std::io::Write;
///
/// let mut temp_file = NamedTempFile::new().unwrap();
/// writeln!(temp_file, "file content").unwrap();
/// let file_path = temp_file.path().to_str().unwrap();
///
/// let contents = read_file(file_path).unwrap();
/// assert_eq!(contents, "file content\n");
/// ```
pub fn read_file(file_path: &str) -> io::Result<String> {
    fs::read_to_string(file_path).map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("Failed to read file: '{}'. {}", file_path, e),
        )
    })
}

#[cfg(test)]
mod read_file_tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_can_read_from_valid_path() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "file content").unwrap();
        let file_path = temp_file.path().to_str().unwrap();

        let contents = read_file(file_path).unwrap();
        assert_eq!(contents, "file content\n");
    }

    #[test]
    fn test_returns_error_with_invalid_path() {
        let result = read_file("invalid_file");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Failed to read file: 'invalid_file'. No such file or directory (os error 2)"
        );
    }
}
