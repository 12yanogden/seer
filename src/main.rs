use replace::{
    find_matches, init_command, verify_has_no_conflicting_options, verify_is_valid_regex,
};

/// The main function that orchestrates the argument parsing, validation, and replacement.
///
/// # Examples
///
/// ```
/// cargo run -- --pattern "foo" --replacement "bar" --haystack "some text"
/// ```
fn main() {
    // Parse arguments
    let matches = init_command().get_matches();
    let pattern = matches.get_one::<String>("pattern").unwrap();
    let replacement = matches.get_one::<String>("replacement").unwrap();
    let haystack = matches.get_one::<String>("haystack").unwrap();
    let replace_all = matches.get_flag("all");
    let nth = matches.get_one::<u16>("nth").copied();
    let every_nth = matches.get_one::<u16>("every_nth").copied();

    // Validate arguments
    verify_is_valid_regex(pattern);
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

    // Find matches
    let matches = find_matches(pattern, haystack);

    // Perform replacement
    let result = if replace_all {
        let mut result = haystack.to_string();
        for (start, end) in matches.iter().rev() {
            result.replace_range(start..end, replacement);
        }
        result
    } else if let Some(nth) = nth {
        if nth == 0 {
            haystack.to_string()
        } else {
            let mut result = haystack.to_string();
            if let Some((start, end)) = matches.get(nth as usize - 1) {
                result.replace_range(start..end, replacement);
            }
            result
        }
    } else if let Some(every_nth) = every_nth {
        if every_nth == 0 {
            haystack.to_string()
        } else {
            let mut result = haystack.to_string();
            for (i, (start, end)) in matches.iter().enumerate().rev() {
                if (i + 1) % every_nth as usize == 0 {
                    result.replace_range(start..end, replacement);
                }
            }
            result
        }
    } else {
        let mut result = haystack.to_string();
        if let Some((start, end)) = matches.first() {
            result.replace_range(start..end, replacement);
        }
        result
    };

    // Output result
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::str::contains;

    #[test]
    fn test_pattern_option_is_required() {
        let mut cmd = Command::cargo_bin("replace").unwrap();
        cmd.arg("--replacement")
            .arg("replacement")
            .arg("--haystack")
            .arg("some text");
        cmd.assert().failure().stderr(contains(
            "the following required arguments were not provided:\n  --pattern <PATTERN>",
        ));
    }

    #[test]
    fn test_replacement_option_is_required() {
        let mut cmd = Command::cargo_bin("replace").unwrap();
        cmd.arg("--pattern")
            .arg("pattern")
            .arg("--haystack")
            .arg("some text");
        cmd.assert().failure().stderr(contains(
            "the following required arguments were not provided:\n  --replacement <REPLACEMENT>",
        ));
    }

    #[test]
    fn test_haystack_option_is_required() {
        let mut cmd = Command::cargo_bin("replace").unwrap();
        cmd.arg("--pattern")
            .arg("pattern")
            .arg("--replacement")
            .arg("replacement");
        cmd.assert().failure().stderr(contains(
            "the following required arguments were not provided:\n  --haystack <HAYSTACK>",
        ));
    }

    #[test]
    fn test_all_option_is_not_required() {
        let mut cmd = Command::cargo_bin("replace").unwrap();
        cmd.arg("--pattern")
            .arg("pattern")
            .arg("--replacement")
            .arg("replacement")
            .arg("--haystack")
            .arg("some text");
        cmd.assert().success();
    }

    #[test]
    fn test_pattern_must_be_valid_regex() {
        let mut cmd = Command::cargo_bin("replace").unwrap();
        cmd.arg("--pattern")
            .arg("[")
            .arg("--replacement")
            .arg("replacement")
            .arg("--haystack")
            .arg("some text");
        cmd.assert().failure().stderr(contains(
            "Error: The pattern given is not a valid regular expression: [",
        ));
    }

    #[test]
    fn test_first_match_is_replaced_without_all_option() {
        let mut cmd = Command::cargo_bin("replace").unwrap();
        cmd.arg("--pattern")
            .arg("pattern")
            .arg("--replacement")
            .arg("replacement")
            .arg("--haystack")
            .arg("pattern pattern");
        cmd.assert()
            .success()
            .stdout(contains("replacement pattern"));
    }

    #[test]
    fn test_every_match_is_replaced_with_all_option() {
        let mut cmd = Command::cargo_bin("replace").unwrap();
        cmd.arg("--pattern")
            .arg("pattern")
            .arg("--replacement")
            .arg("replacement")
            .arg("--haystack")
            .arg("pattern pattern")
            .arg("--all");
        cmd.assert()
            .success()
            .stdout(contains("replacement replacement"));
    }

    #[test]
    fn test_no_replacements_with_every_nth_zero() {
        let mut cmd = Command::cargo_bin("replace").unwrap();
        cmd.arg("--pattern")
            .arg("pattern")
            .arg("--replacement")
            .arg("replacement")
            .arg("--haystack")
            .arg("pattern pattern pattern pattern")
            .arg("--every_nth")
            .arg("0");
        cmd.assert()
            .success()
            .stdout(contains("pattern pattern pattern pattern"));
    }

    #[test]
    fn test_every_match_is_replaced_with_every_nth_one() {
        let mut cmd = Command::cargo_bin("replace").unwrap();
        cmd.arg("--pattern")
            .arg("pattern")
            .arg("--replacement")
            .arg("replacement")
            .arg("--haystack")
            .arg("pattern pattern pattern pattern")
            .arg("--every_nth")
            .arg("1");
        cmd.assert()
            .success()
            .stdout(contains("replacement replacement replacement replacement"));
    }

    #[test]
    fn test_every_second_match_is_replaced_with_every_nth_option() {
        let mut cmd = Command::cargo_bin("replace").unwrap();
        cmd.arg("--pattern")
            .arg("pattern")
            .arg("--replacement")
            .arg("replacement")
            .arg("--haystack")
            .arg("pattern pattern pattern pattern")
            .arg("--every_nth")
            .arg("2");
        cmd.assert()
            .success()
            .stdout(contains("pattern replacement pattern replacement"));
    }

    #[test]
    fn test_every_third_match_is_replaced_with_every_nth_option() {
        let mut cmd = Command::cargo_bin("replace").unwrap();
        cmd.arg("--pattern")
            .arg("pattern")
            .arg("--replacement")
            .arg("replacement")
            .arg("--haystack")
            .arg("pattern pattern pattern pattern pattern pattern")
            .arg("--every_nth")
            .arg("3");
        cmd.assert().success().stdout(contains(
            "pattern pattern replacement pattern pattern replacement",
        ));
    }

    #[test]
    fn test_all_and_every_nth_options_cannot_be_provided_together() {
        let mut cmd = Command::cargo_bin("replace").unwrap();
        cmd.arg("--pattern")
            .arg("pattern")
            .arg("--replacement")
            .arg("replacement")
            .arg("--haystack")
            .arg("some text")
            .arg("--all")
            .arg("--every_nth")
            .arg("2");
        cmd.assert().failure().stderr(contains(
            "Error: Conflicting options provided: \"all\", \"every_nth\"",
        ));
    }

    #[test]
    fn test_no_replacements_with_nth_zero() {
        let mut cmd = Command::cargo_bin("replace").unwrap();
        cmd.arg("--pattern")
            .arg("pattern")
            .arg("--replacement")
            .arg("replacement")
            .arg("--haystack")
            .arg("pattern pattern pattern pattern")
            .arg("--nth")
            .arg("0");
        cmd.assert()
            .success()
            .stdout(contains("pattern pattern pattern pattern"));
    }

    #[test]
    fn test_first_match_is_replaced_with_nth_one() {
        let mut cmd = Command::cargo_bin("replace").unwrap();
        cmd.arg("--pattern")
            .arg("pattern")
            .arg("--replacement")
            .arg("replacement")
            .arg("--haystack")
            .arg("pattern pattern pattern pattern")
            .arg("--nth")
            .arg("1");
        cmd.assert()
            .success()
            .stdout(contains("replacement pattern pattern pattern"));
    }

    #[test]
    fn test_second_match_is_replaced_with_nth_two() {
        let mut cmd = Command::cargo_bin("replace").unwrap();
        cmd.arg("--pattern")
            .arg("pattern")
            .arg("--replacement")
            .arg("replacement")
            .arg("--haystack")
            .arg("pattern pattern pattern pattern")
            .arg("--nth")
            .arg("2");
        cmd.assert()
            .success()
            .stdout(contains("pattern replacement pattern pattern"));
    }

    #[test]
    fn test_nth_option_does_not_accept_negative_numbers() {
        let mut cmd = Command::cargo_bin("replace").unwrap();
        cmd.arg("--pattern")
            .arg("pattern")
            .arg("--replacement")
            .arg("replacement")
            .arg("--haystack")
            .arg("some text")
            .arg("--nth=-1");
        cmd.assert().failure().stderr(contains(
            "error: invalid value \'-1\' for \'--nth <NTH>\': -1 is not in 0..=65535",
        ));
    }

    #[test]
    fn test_every_nth_option_does_not_accept_negative_numbers() {
        let mut cmd = Command::cargo_bin("replace").unwrap();
        cmd.arg("--pattern")
            .arg("pattern")
            .arg("--replacement")
            .arg("replacement")
            .arg("--haystack")
            .arg("some text")
            .arg("--every_nth=-1");
        cmd.assert().failure().stderr(contains(
            "error: invalid value \'-1\' for \'--every_nth <EVERY_NTH>\': -1 is not in 0..=65535",
        ));
    }

    #[test]
    fn test_nth_option_does_not_accept_floats() {
        let mut cmd = Command::cargo_bin("replace").unwrap();
        cmd.arg("--pattern")
            .arg("pattern")
            .arg("--replacement")
            .arg("replacement")
            .arg("--haystack")
            .arg("some text")
            .arg("--nth")
            .arg("1.5");
        cmd.assert().failure().stderr(contains(
            "error: invalid value \'1.5\' for \'--nth <NTH>\': invalid digit found in string",
        ));
    }

    #[test]
    fn test_every_nth_option_does_not_accept_floats() {
        let mut cmd = Command::cargo_bin("replace").unwrap();
        cmd.arg("--pattern")
            .arg("pattern")
            .arg("--replacement")
            .arg("replacement")
            .arg("--haystack")
            .arg("some text")
            .arg("--every_nth")
            .arg("1.5");
        cmd.assert().failure().stderr(contains(
            "error: invalid value \'1.5\' for \'--every_nth <EVERY_NTH>\': invalid digit found in string",
        ));
    }

    #[test]
    fn test_nth_option_does_not_accept_text() {
        let mut cmd = Command::cargo_bin("replace").unwrap();
        cmd.arg("--pattern")
            .arg("pattern")
            .arg("--replacement")
            .arg("replacement")
            .arg("--haystack")
            .arg("some text")
            .arg("--nth")
            .arg("text");
        cmd.assert().failure().stderr(contains(
            "error: invalid value \'text\' for \'--nth <NTH>\': invalid digit found in string",
        ));
    }

    #[test]
    fn test_every_nth_option_does_not_accept_text() {
        let mut cmd = Command::cargo_bin("replace").unwrap();
        cmd.arg("--pattern")
            .arg("pattern")
            .arg("--replacement")
            .arg("replacement")
            .arg("--haystack")
            .arg("some text")
            .arg("--every_nth")
            .arg("text");
        cmd.assert().failure().stderr(contains(
            "error: invalid value \'text\' for \'--every_nth <EVERY_NTH>\': invalid digit found in string",
        ));
    }
}
