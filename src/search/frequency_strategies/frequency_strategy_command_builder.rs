use clap::{Arg, ArgGroup, Command};

pub struct FrequencyStrategyCommandBuilder;

impl CommandBuilder for FrequencyStrategyCommandBuilder {
    fn build(mut cmd: Command) -> Command {
        cmd.arg(
            Arg::new("nth")
                .long("nth")
                .value_name("NTH")
                .help("Find/edit only the nth match")
                .value_parser(clap::value_parser!(u64).range(0..))
                .action(clap::ArgAction::Set),
        )
        .arg(
            Arg::new("every_nth")
                .long("every_nth")
                .value_name("EVERY_NTH")
                .help("Find/edit every nth match")
                .value_parser(clap::value_parser!(u64).range(0..))
                .action(clap::ArgAction::Set),
        )
        .arg(
            Arg::new("all")
                .long("all")
                .value_name("ALL")
                .help("Find/edit all matches")
                .action(clap::ArgAction::SetTrue),
        )
        .group(
            ArgGroup::new("frequency")
                .args(["nth", "every_nth", "all"])
                .required(false)
                .multiple(false),
        )
    }
}

#[cfg(test)]
mod frequency_strategy_tests {
    use super::*;
    use clap::error::ErrorKind;

    lazy_static::lazy_static! {
        static ref CMD: Command = {
            let cmd = Command::new("test");
            FrequencyStrategyCommandBuilder::build(cmd)
        };
    }

    #[test]
    fn test_zero_or_one_frequencies_can_be_given() {
        // Test with no frequency
        let matches = CMD.clone().try_get_matches_from(vec!["test"]);
        assert!(matches.is_ok());

        // Test with only --nth
        let matches = CMD.clone().try_get_matches_from(vec!["test", "--nth", "1"]);
        assert!(matches.is_ok());

        // Test with only --every_nth
        let matches = CMD
            .clone()
            .try_get_matches_from(vec!["test", "--every_nth", "2"]);
        assert!(matches.is_ok());

        // Test with only --all
        let matches = CMD.clone().try_get_matches_from(vec!["test", "--all"]);
        assert!(matches.is_ok());

        // Test with --nth and --every_nth (should fail)
        let matches =
            CMD.clone()
                .try_get_matches_from(vec!["test", "--nth", "1", "--every_nth", "2"]);
        assert!(matches.is_err());
        let err = matches.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::ArgumentConflict);

        // Test with --nth and --all (should fail)
        let matches = CMD
            .clone()
            .try_get_matches_from(vec!["test", "--nth", "1", "--all"]);
        assert!(matches.is_err());
        let err = matches.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::ArgumentConflict);

        // Test with --every_nth and --all (should fail)
        let matches = CMD
            .clone()
            .try_get_matches_from(vec!["test", "--every_nth", "2", "--all"]);
        assert!(matches.is_err());
        let err = matches.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::ArgumentConflict);
    }
}
