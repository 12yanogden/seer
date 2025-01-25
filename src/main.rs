use regex::Regex;
use replace_in_file::{init_command, validate_file, validate_pattern};
use std::fs;

/// The main function that orchestrates the argument parsing, validation, and replacement.
///
/// # Examples
///
/// ```
/// cargo run -- --pattern "foo" --replacement "bar" --file "test.txt"
/// ```
fn main() {
    // Parse arguments
    let matches = init_command().get_matches();
    let pattern = matches.get_one::<String>("pattern").unwrap();
    let replacement = matches.get_one::<String>("replacement").unwrap();
    let file = matches.get_one::<String>("file").unwrap();
    let replace_all = matches.get_flag("all");

    // Validate arguments
    validate_pattern(pattern);
    validate_file(file);

    // Process arguments
    let content = fs::read_to_string(file).expect(&format!("Could not read file: {}", file));
    let re = Regex::new(pattern).expect(&format!("Invalid regex pattern: {}", pattern));

    // Perform replacement
    let result = if replace_all {
        re.replace_all(&content, replacement)
    } else {
        re.replace(&content, replacement)
    };

    // Write to file
    fs::write(file, result.as_ref()).expect(&format!("Could not write to file: {}", file));
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::str::contains;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_pattern_option_is_required() {
        let mut cmd = Command::cargo_bin("replace_in_file").unwrap();
        cmd.arg("--replacement")
            .arg("replacement")
            .arg("--file")
            .arg("test.txt");
        cmd.assert().failure().stderr(contains(
            "the following required arguments were not provided:\n  --pattern <PATTERN>",
        ));
    }

    #[test]
    fn test_replacement_option_is_required() {
        let mut cmd = Command::cargo_bin("replace_in_file").unwrap();
        cmd.arg("--pattern")
            .arg("pattern")
            .arg("--file")
            .arg("test.txt");
        cmd.assert().failure().stderr(contains(
            "the following required arguments were not provided:\n  --replacement <REPLACEMENT>",
        ));
    }

    #[test]
    fn test_file_option_is_required() {
        let mut cmd = Command::cargo_bin("replace_in_file").unwrap();
        cmd.arg("--pattern")
            .arg("pattern")
            .arg("--replacement")
            .arg("replacement");
        cmd.assert().failure().stderr(contains(
            "the following required arguments were not provided:\n  --file <FILE>",
        ));
    }

    #[test]
    fn test_all_option_is_not_required() {
        let mut cmd = Command::cargo_bin("replace_in_file").unwrap();
        cmd.arg("--pattern")
            .arg("pattern")
            .arg("--replacement")
            .arg("replacement")
            .arg("--file")
            .arg("test.txt");
        cmd.assert()
            .failure()
            .stderr(contains("The file given does not exist: test.txt"));
    }

    #[test]
    fn test_pattern_must_be_valid_regex() {
        let mut cmd = Command::cargo_bin("replace_in_file").unwrap();
        cmd.arg("--pattern")
            .arg("[")
            .arg("--replacement")
            .arg("replacement")
            .arg("--file")
            .arg("test.txt");
        cmd.assert().failure().stderr(contains(
            "Error: The pattern given is not a valid regular expression: [",
        ));
    }

    #[test]
    fn test_file_must_be_valid_path() {
        let mut cmd = Command::cargo_bin("replace_in_file").unwrap();
        cmd.arg("--pattern")
            .arg("pattern")
            .arg("--replacement")
            .arg("replacement")
            .arg("--file")
            .arg("invalid_file.txt");
        cmd.assert()
            .failure()
            .stderr(contains("The file given does not exist: invalid_file.txt"));
    }

    #[test]
    fn test_first_match_is_replaced_without_all_option() {
        let file = NamedTempFile::new().unwrap();
        fs::write(file.path(), "pattern pattern").unwrap();

        let mut cmd = Command::cargo_bin("replace_in_file").unwrap();
        cmd.arg("--pattern")
            .arg("pattern")
            .arg("--replacement")
            .arg("replacement")
            .arg("--file")
            .arg(file.path());
        cmd.assert().success();

        let content = fs::read_to_string(file.path()).unwrap();
        assert_eq!(content, "replacement pattern");
    }

    #[test]
    fn test_every_match_is_replaced_with_all_option() {
        let file = NamedTempFile::new().unwrap();
        fs::write(file.path(), "pattern pattern").unwrap();

        let mut cmd = Command::cargo_bin("replace_in_file").unwrap();
        cmd.arg("--pattern")
            .arg("pattern")
            .arg("--replacement")
            .arg("replacement")
            .arg("--file")
            .arg(file.path())
            .arg("--all");
        cmd.assert().success();

        let content = fs::read_to_string(file.path()).unwrap();
        assert_eq!(content, "replacement replacement");
    }
}
