use super::all_frequency_strategy::AllFrequencyStrategy;
use super::every_nth_frequency_strategy::EveryNthFrequencyStrategy;
use super::frequency_strategy::FrequencyStrategy;
use super::nth_frequency_strategy::NthFrequencyStrategy;
use clap::ArgMatches;

/// Factory for creating frequency strategies based on command line arguments.
pub struct FrequencyStrategyFactory;

impl FrequencyStrategyFactory {
    /// Creates a frequency strategy based on the provided command line arguments.
    ///
    /// # Arguments
    ///
    /// * `inputs` - A reference to `ArgMatches` containing the parsed command line arguments.
    ///
    /// # Returns
    ///
    /// A boxed `FrequencyStrategy` instance.
    ///
    /// # Panics
    ///
    /// Panics if no valid frequency strategy is provided.
    ///
    /// # Examples
    ///
    /// ```
    /// use clap::{Command, Arg};
    /// use parse::frequency_strategies::frequency_strategy_factory::FrequencyStrategyFactory;
    /// use parse::frequency_strategies::frequency_strategy_type::FrequencyStrategyType;
    ///
    /// let cmd = Command::new("test")
    ///     .arg(Arg::new("nth")
    ///     .long("nth")
    ///     .value_name("NTH")
    ///     .value_parser(clap::value_parser!(u64))
    ///     .action(clap::ArgAction::Set));
    /// let matches = cmd.try_get_matches_from(vec!["test", "--nth", "3"]).unwrap();
    ///
    /// let strategy = FrequencyStrategyFactory::make(&matches);
    /// assert_eq!(strategy.strategy_type(), FrequencyStrategyType::Nth);
    /// ```
    pub fn make(inputs: &ArgMatches) -> Box<dyn FrequencyStrategy> {
        if let Some(nth) = inputs.get_one::<u64>("nth") {
            return Box::new(NthFrequencyStrategy::new(*nth as usize));
        } else if let Some(every_nth) = inputs.get_one::<u64>("every_nth") {
            return Box::new(EveryNthFrequencyStrategy::new(*every_nth as usize, 0));
        } else if inputs.get_flag("all") {
            return Box::new(AllFrequencyStrategy::new());
        } else {
            return Box::new(NthFrequencyStrategy::new(1));
        }
    }

    /// Creates a frequency strategy for testing purposes.
    ///
    /// # Returns
    ///
    /// A boxed `FrequencyStrategy` instance that always returns an `AllFrequencyStrategy`.
    ///
    /// # Examples
    ///
    /// ```
    /// use parse::frequency_strategies::frequency_strategy_factory::FrequencyStrategyFactory;
    /// use parse::frequency_strategies::frequency_strategy_type::FrequencyStrategyType;
    ///
    /// let strategy = FrequencyStrategyFactory::make_for_testing();
    /// assert_eq!(strategy.strategy_type(), FrequencyStrategyType::All);
    /// ```
    pub fn make_for_testing() -> Box<dyn FrequencyStrategy> {
        Box::new(AllFrequencyStrategy::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frequency_strategies::frequency_strategy_type::FrequencyStrategyType;
    use clap::{Arg, Command};

    fn get_command() -> Command {
        Command::new("test")
            .arg(
                Arg::new("nth")
                    .long("nth")
                    .value_name("NTH")
                    .value_parser(clap::value_parser!(u64))
                    .action(clap::ArgAction::Set),
            )
            .arg(
                Arg::new("every_nth")
                    .long("every_nth")
                    .value_name("EVERY_NTH")
                    .value_parser(clap::value_parser!(u64))
                    .action(clap::ArgAction::Set),
            )
            .arg(Arg::new("all").long("all").action(clap::ArgAction::SetTrue))
    }

    #[test]
    fn test_nth_frequency_strategy() {
        let cmd = get_command();
        let matches = cmd
            .try_get_matches_from(vec!["test", "--nth", "3"])
            .unwrap();
        let strategy = FrequencyStrategyFactory::make(&matches);
        assert_eq!(strategy.strategy_type(), FrequencyStrategyType::Nth);
    }

    #[test]
    fn test_every_nth_frequency_strategy() {
        let cmd = get_command();
        let matches = cmd
            .try_get_matches_from(vec!["test", "--every_nth", "2"])
            .unwrap();
        let strategy = FrequencyStrategyFactory::make(&matches);
        assert_eq!(strategy.strategy_type(), FrequencyStrategyType::EveryNth);
    }

    #[test]
    fn test_all_frequency_strategy() {
        let cmd = get_command();
        let matches = cmd.try_get_matches_from(vec!["test", "--all"]).unwrap();
        let strategy = FrequencyStrategyFactory::make(&matches);
        assert_eq!(strategy.strategy_type(), FrequencyStrategyType::All);
    }

    #[test]
    fn test_default_strategy() {
        let cmd = get_command();
        let matches = cmd.try_get_matches_from(vec!["test"]).unwrap();
        let strategy = FrequencyStrategyFactory::make(&matches);
        assert_eq!(strategy.strategy_type(), FrequencyStrategyType::Nth);
    }
}
