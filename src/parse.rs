use crate::read_file;
use clap::ArgMatches;
use std::io::{self, Read};

/// Extracts the searchable body of text from the `--text` option, `--file` option, or piped input.
///
/// # Arguments
///
/// * `matches` - A reference to the `clap::ArgMatches` instance.
/// * `pipe` - An `Option<String>` containing the piped input.
///
/// # Returns
///
/// A `Result<String, io::Error>` containing the searchable body of text or an error.
///
/// # Examples
///
/// ```
/// use parse::{init, get_searchable};
/// let matches = init().try_get_matches_from(vec!["parse", "--exact", "foo", "--text", "bar"]);
/// assert!(matches.is_ok());
/// let pipe = None;
/// let searchable = get_searchable(&matches.unwrap(), &pipe).unwrap();
/// assert_eq!(searchable, "bar");
/// ```
pub fn get_searchable(matches: &ArgMatches, pipe: &Option<String>) -> io::Result<String> {
    if let Some(text) = matches.get_one::<String>("text") {
        Ok(text.clone())
    } else if let Some(file_path) = matches.get_one::<String>("file") {
        read_file(file_path)
    } else if let Some(pipe_input) = pipe {
        Ok(pipe_input.clone())
    } else {
        // This branch should never be reached because input validation ensures that one of the options is provided.
        Ok(String::new())
    }
}

#[cfg(test)]
mod get_searchable_tests {
    use crate::init;

    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_get_searchable_with_text_option() {
        let matches = init()
            .try_get_matches_from(vec!["parse", "--exact", "foo", "--text", "some text"])
            .unwrap();
        let pipe = None;
        let searchable = get_searchable(&matches, &pipe).unwrap();
        assert_eq!(searchable, "some text");
    }

    #[test]
    fn test_get_searchable_with_file_option() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "file content").unwrap();
        let file_path = temp_file.path().to_str().unwrap();

        let matches = init()
            .try_get_matches_from(vec!["parse", "--exact", "foo", "--file", file_path])
            .unwrap();
        let pipe = None;
        let searchable = get_searchable(&matches, &pipe).unwrap();
        assert_eq!(searchable, "file content\n");
    }

    #[test]
    fn test_get_searchable_with_piped_input() {
        let matches = init()
            .try_get_matches_from(vec!["parse", "--exact", "foo"])
            .unwrap();
        let pipe = Some(String::from("piped input"));
        let searchable = get_searchable(&matches, &pipe).unwrap();
        assert_eq!(searchable, "piped input");
    }
}

/// Reads piped input from stdin.
///
/// # Returns
///
/// An `Option<String>` containing the piped input if available.
///
pub fn read_pipe() -> Option<String> {
    let mut pipe = String::new();
    let could_read_input = io::stdin().read_to_string(&mut pipe).is_ok();

    if could_read_input && !pipe.is_empty() {
        Some(pipe)
    } else {
        None
    }
}
