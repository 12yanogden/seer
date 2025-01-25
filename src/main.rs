use regex::Regex;
use replace_in_file::{
    init_command, verify_file_path_exists, verify_has_no_conflicting_options,
    verify_is_positive_int, verify_is_valid_regex,
};
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
    let nth = matches.get_one::<String>("nth").map(|s| {
        verify_is_positive_int(s);
        s.parse::<usize>().unwrap()
    });
    let every_nth = matches.get_one::<String>("every_nth").map(|s| {
        verify_is_positive_int(s);
        s.parse::<usize>().unwrap()
    });

    // Validate arguments
    verify_is_valid_regex(pattern);
    verify_file_path_exists(file);
    verify_has_no_conflicting_options(vec![
        (
            if replace_all { Some("all") } else { None },
            every_nth.map(|_| "every_nth"),
        ),
        (
            if replace_all { Some("all") } else { None },
            nth.map(|_| "nth"),
        ),
        (every_nth.map(|_| "every_nth"), nth.map(|_| "nth")),
    ]);

    // Process arguments
    let content = fs::read_to_string(file).expect(&format!("Could not read file: {}", file));
    let re = Regex::new(pattern).expect(&format!("Invalid regex pattern: {}", pattern));

    // Perform replacement
    let result = if replace_all {
        re.replace_all(&content, replacement)
    } else if let Some(nth) = nth {
        if nth == 0 {
            content.into()
        } else {
            let mut count = 0;
            re.replace_all(&content, |caps: &regex::Captures| {
                count += 1;
                if count == nth {
                    replacement.to_string()
                } else {
                    caps[0].to_string()
                }
            })
        }
    } else if let Some(every_nth) = every_nth {
        if every_nth == 0 {
            content.into()
        } else if every_nth == 1 {
            re.replace_all(&content, replacement)
        } else {
            let mut count = 0;
            re.replace_all(&content, |caps: &regex::Captures| {
                count += 1;
                if count % every_nth == 0 {
                    replacement.to_string()
                } else {
                    caps[0].to_string()
                }
            })
        }
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

    #[test]
    fn test_no_replacements_with_every_nth_zero() {
        let file = NamedTempFile::new().unwrap();
        fs::write(file.path(), "pattern pattern pattern pattern").unwrap();

        let mut cmd = Command::cargo_bin("replace_in_file").unwrap();
        cmd.arg("--pattern")
            .arg("pattern")
            .arg("--replacement")
            .arg("replacement")
            .arg("--file")
            .arg(file.path())
            .arg("--every_nth")
            .arg("0");
        cmd.assert().success();

        let content = fs::read_to_string(file.path()).unwrap();
        assert_eq!(content, "pattern pattern pattern pattern");
    }

    #[test]
    fn test_every_match_is_replaced_with_every_nth_one() {
        let file = NamedTempFile::new().unwrap();
        fs::write(file.path(), "pattern pattern pattern pattern").unwrap();

        let mut cmd = Command::cargo_bin("replace_in_file").unwrap();
        cmd.arg("--pattern")
            .arg("pattern")
            .arg("--replacement")
            .arg("replacement")
            .arg("--file")
            .arg(file.path())
            .arg("--every_nth")
            .arg("1");
        cmd.assert().success();

        let content = fs::read_to_string(file.path()).unwrap();
        assert_eq!(content, "replacement replacement replacement replacement");
    }

    #[test]
    fn test_every_second_match_is_replaced_with_every_nth_option() {
        let file = NamedTempFile::new().unwrap();
        fs::write(file.path(), "pattern pattern pattern pattern").unwrap();

        let mut cmd = Command::cargo_bin("replace_in_file").unwrap();
        cmd.arg("--pattern")
            .arg("pattern")
            .arg("--replacement")
            .arg("replacement")
            .arg("--file")
            .arg(file.path())
            .arg("--every_nth")
            .arg("2");
        cmd.assert().success();

        let content = fs::read_to_string(file.path()).unwrap();
        assert_eq!(content, "pattern replacement pattern replacement");
    }

    #[test]
    fn test_every_third_match_is_replaced_with_every_nth_option() {
        let file = NamedTempFile::new().unwrap();
        fs::write(
            file.path(),
            "pattern pattern pattern pattern pattern pattern",
        )
        .unwrap();

        let mut cmd = Command::cargo_bin("replace_in_file").unwrap();
        cmd.arg("--pattern")
            .arg("pattern")
            .arg("--replacement")
            .arg("replacement")
            .arg("--file")
            .arg(file.path())
            .arg("--every_nth")
            .arg("3");
        cmd.assert().success();

        let content = fs::read_to_string(file.path()).unwrap();
        assert_eq!(
            content,
            "pattern pattern replacement pattern pattern replacement"
        );
    }

    #[test]
    fn test_all_and_every_nth_options_cannot_be_provided_together() {
        let file = NamedTempFile::new().unwrap();
        let mut cmd = Command::cargo_bin("replace_in_file").unwrap();
        cmd.arg("--pattern")
            .arg("pattern")
            .arg("--replacement")
            .arg("replacement")
            .arg("--file")
            .arg(file.path())
            .arg("--all")
            .arg("--every_nth")
            .arg("2");
        cmd.assert().failure().stderr(contains(
            "Error: Conflicting options provided: \"all\", \"every_nth\"",
        ));
    }

    #[test]
    fn test_no_replacements_with_nth_zero() {
        let file = NamedTempFile::new().unwrap();
        fs::write(file.path(), "pattern pattern pattern pattern").unwrap();

        let mut cmd = Command::cargo_bin("replace_in_file").unwrap();
        cmd.arg("--pattern")
            .arg("pattern")
            .arg("--replacement")
            .arg("replacement")
            .arg("--file")
            .arg(file.path())
            .arg("--nth")
            .arg("0");
        cmd.assert().success();

        let content = fs::read_to_string(file.path()).unwrap();
        assert_eq!(content, "pattern pattern pattern pattern");
    }

    #[test]
    fn test_first_match_is_replaced_with_nth_one() {
        let file = NamedTempFile::new().unwrap();
        fs::write(file.path(), "pattern pattern pattern pattern").unwrap();

        let mut cmd = Command::cargo_bin("replace_in_file").unwrap();
        cmd.arg("--pattern")
            .arg("pattern")
            .arg("--replacement")
            .arg("replacement")
            .arg("--file")
            .arg(file.path())
            .arg("--nth")
            .arg("1");
        cmd.assert().success();

        let content = fs::read_to_string(file.path()).unwrap();
        assert_eq!(content, "replacement pattern pattern pattern");
    }

    #[test]
    fn test_second_match_is_replaced_with_nth_two() {
        let file = NamedTempFile::new().unwrap();
        fs::write(file.path(), "pattern pattern pattern pattern").unwrap();

        let mut cmd = Command::cargo_bin("replace_in_file").unwrap();
        cmd.arg("--pattern")
            .arg("pattern")
            .arg("--replacement")
            .arg("replacement")
            .arg("--file")
            .arg(file.path())
            .arg("--nth")
            .arg("2");
        cmd.assert().success();

        let content = fs::read_to_string(file.path()).unwrap();
        assert_eq!(content, "pattern replacement pattern pattern");
    }

    #[test]
    fn test_nth_option_does_not_accept_negative_numbers() {
        let mut cmd = Command::cargo_bin("replace_in_file").unwrap();
        cmd.arg("--pattern")
            .arg("pattern")
            .arg("--replacement")
            .arg("replacement")
            .arg("--file")
            .arg("test.txt")
            .arg("--nth=-1");
        cmd.assert().failure().stderr(contains(
            "Error: The value given is not a positive integer: -1",
        ));
    }

    #[test]
    fn test_every_nth_option_does_not_accept_negative_numbers() {
        let mut cmd = Command::cargo_bin("replace_in_file").unwrap();
        cmd.arg("--pattern")
            .arg("pattern")
            .arg("--replacement")
            .arg("replacement")
            .arg("--file")
            .arg("test.txt")
            .arg("--every_nth=-1");
        cmd.assert().failure().stderr(contains(
            "Error: The value given is not a positive integer: -1",
        ));
    }

    #[test]
    fn test_nth_option_does_not_accept_floats() {
        let mut cmd = Command::cargo_bin("replace_in_file").unwrap();
        cmd.arg("--pattern")
            .arg("pattern")
            .arg("--replacement")
            .arg("replacement")
            .arg("--file")
            .arg("test.txt")
            .arg("--nth")
            .arg("1.5");
        cmd.assert().failure().stderr(contains(
            "Error: The value given is not a positive integer: 1.5",
        ));
    }

    #[test]
    fn test_every_nth_option_does_not_accept_floats() {
        let mut cmd = Command::cargo_bin("replace_in_file").unwrap();
        cmd.arg("--pattern")
            .arg("pattern")
            .arg("--replacement")
            .arg("replacement")
            .arg("--file")
            .arg("test.txt")
            .arg("--every_nth")
            .arg("1.5");
        cmd.assert().failure().stderr(contains(
            "Error: The value given is not a positive integer: 1.5",
        ));
    }

    #[test]
    fn test_nth_option_does_not_accept_text() {
        let mut cmd = Command::cargo_bin("replace_in_file").unwrap();
        cmd.arg("--pattern")
            .arg("pattern")
            .arg("--replacement")
            .arg("replacement")
            .arg("--file")
            .arg("test.txt")
            .arg("--nth")
            .arg("text");
        cmd.assert().failure().stderr(contains(
            "Error: The value given is not a positive integer: text",
        ));
    }

    #[test]
    fn test_every_nth_option_does_not_accept_text() {
        let mut cmd = Command::cargo_bin("replace_in_file").unwrap();
        cmd.arg("--pattern")
            .arg("pattern")
            .arg("--replacement")
            .arg("replacement")
            .arg("--file")
            .arg("test.txt")
            .arg("--every_nth")
            .arg("text");
        cmd.assert().failure().stderr(contains(
            "Error: The value given is not a positive integer: text",
        ));
    }
}
