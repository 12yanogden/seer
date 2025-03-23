use clap::{error::ErrorKind, ArgMatches};

/// Validates the input arguments.
///
/// # Arguments
///
/// * `matches` - A reference to the `clap::ArgMatches` instance.
/// * `pipe` - An `Option<String>` containing the piped input.
///
/// # Returns
///
/// A `Result` which is `Ok(())` if the validation passes, or an `Err` with `ErrorKind::MissingRequiredArgument` if it fails.
///
/// # Examples
///
/// ```
/// use clap::error::ErrorKind;
/// use parse::{init, validate_input};
///
/// let matches = init().try_get_matches_from(vec!["parse", "--exact", "foo", "--text", "bar"]);
/// assert!(matches.is_ok());
/// let pipe = Some(String::from("piped input"));
/// let matches = validate_input(&matches.unwrap(), &pipe);
/// assert!(matches.is_ok());
/// ```
pub fn validate_input(matches: &ArgMatches, pipe: &Option<String>) -> Result<(), clap::Error> {
    verify_searchable_or_pipe_is_given(matches, pipe)?;
    verify_required_option_for_dependent_flag(matches, "between", "exclude_matches")?;
    verify_required_option_for_dependent_flag(matches, "file", "in_place")?;
    verify_required_option_for_dependent_flag(matches, "dir", "max_depth")?;
    Ok(())
}

/// Verifies that either a searchable argument or piped input is provided.
///
/// # Arguments
///
/// * `matches` - A reference to the `clap::ArgMatches` instance.
/// * `pipe` - An `Option<String>` containing the piped input.
///
/// # Returns
///
/// A `Result` which is `Ok(())` if the validation passes, or an `Err` with `ErrorKind::MissingRequiredArgument` if it fails.
///
/// # Examples
///
/// ```
/// use clap::error::ErrorKind;
/// use parse::{init, verify_searchable_or_pipe_is_given};
///
/// let matches = init().try_get_matches_from(vec!["parse", "--exact", "foo"]);
/// assert!(matches.is_ok());
/// let binding = matches.unwrap();
/// let pipe = Some(String::from("piped input"));
/// let result = verify_searchable_or_pipe_is_given(&binding, &pipe);
/// assert!(result.is_ok());
/// ```
pub fn verify_searchable_or_pipe_is_given<'a>(
    matches: &'a clap::ArgMatches,
    pipe: &Option<String>,
) -> Result<&'a clap::ArgMatches, clap::Error> {
    if matches.contains_id("text")
        || matches.contains_id("file")
        || matches.contains_id("dir")
        || pipe.is_some()
    {
        Ok(matches)
    } else {
        Err(clap::Error::raw(
            ErrorKind::MissingRequiredArgument,
            "Either a searchable argument (text, file, or dir) or piped input must be provided.",
        ))
    }
}

#[cfg(test)]
mod verify_searchable_or_pipe_is_given_tests {
    use clap::Command;

    use crate::init;

    use super::*;

    lazy_static::lazy_static! {
        static ref CMD: Command = init();
    }

    #[test]
    fn test_throw_missing_required_argument_if_no_searchable_given() {
        let matches = init().try_get_matches_from(vec!["parse", "--exact", "foo"]);
        assert!(matches.is_ok());
        let binding = matches.unwrap();
        let result = verify_searchable_or_pipe_is_given(&binding, &None);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);
    }
}

/// Verifies that a flag is used with a required option.
///
/// This function is only necessary because of GitHub issue #4707:
/// https://github.com/clap-rs/clap/issues/4707
///
/// # Arguments
///
/// * `matches` - A reference to the `clap::ArgMatches` instance.
/// * `required_option` - The option required by the flag.
/// * `dependent_flag` - The flag that requires the option.
///
/// # Returns
///
/// A `Result` which is `Ok(&clap::ArgMatches)` if the validation passes, or an `Err` with `ErrorKind::MissingRequiredArgument` if it fails.
///
/// # Examples
///
/// ```
/// use clap::{Arg, ArgGroup, Command, error::ErrorKind};
/// use parse::{init, verify_required_option_for_dependent_flag};
///
/// let matches = init().try_get_matches_from(vec!["parse", "--between", "foo", "bar", "--text", "bar", "--exclude_matches"]);
/// assert!(matches.is_ok());
/// let binding = matches.unwrap();
/// let result = verify_required_option_for_dependent_flag(&binding, "between", "exclude_matches");
/// assert!(result.is_ok());
/// ```
pub fn verify_required_option_for_dependent_flag<'a>(
    matches: &'a clap::ArgMatches,
    required_option: &str,
    dependent_flag: &str,
) -> Result<&'a clap::ArgMatches, clap::Error> {
    if matches.get_one::<bool>(dependent_flag) == Some(&true)
        && !matches.contains_id(required_option)
    {
        return Err(clap::Error::raw(
            ErrorKind::MissingRequiredArgument,
            format!(
                "The '--{}' option requires the '--{}' option.",
                dependent_flag, required_option
            ),
        ));
    }
    Ok(matches)
}

#[cfg(test)]
mod verify_required_option_for_dependent_flag_tests {
    use clap::Command;

    use crate::init;

    use super::*;

    lazy_static::lazy_static! {
        static ref CMD: Command = init();
    }

    #[test]
    fn test_throw_missing_required_argument_if_no_required_option_given() {
        // Test with only --exclude_matches
        let matches = CMD.clone().try_get_matches_from(vec![
            "parse",
            "--exact",
            "foo",
            "--text",
            "bar",
            "--exclude_matches",
        ]);
        assert!(matches.is_ok());
        let binding = matches.unwrap();
        let matches =
            verify_required_option_for_dependent_flag(&binding, "between", "exclude_matches");
        assert!(matches.is_err());
        let err = matches.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);
    }

    #[test]
    fn test_return_ok_without_dependent_flag() {
        // Test without --exclude_matches
        let matches = CMD
            .clone()
            .try_get_matches_from(vec!["parse", "--exact", "foo", "--text", "bar"]);
        assert!(matches.is_ok());
        let binding = matches.unwrap();
        let matches =
            verify_required_option_for_dependent_flag(&binding, "between", "exclude_matches");
        assert!(matches.is_ok());
    }
}
