use crate::frequency_strategies::frequency_strategy::FrequencyStrategy;

use super::between_search_strategy::BetweenSearchStrategy;
use super::exact_search_strategy::ExactSearchStrategy;
use super::regex_search_strategy::RegexSearchStrategy;
use super::search_strategy::SearchStrategy;
use clap::ArgMatches;

/// Factory for creating search strategies based on command line arguments.
pub struct SearchStrategyFactory;

impl SearchStrategyFactory {
    /// Creates a search strategy based on the provided command line arguments.
    ///
    /// # Arguments
    ///
    /// * `inputs` - A reference to `ArgMatches` containing the seerd command line arguments.
    /// * `frequency_strategy` - A frequency strategy to be used by the search strategies.
    ///
    /// # Returns
    ///
    /// A boxed `SearchStrategy` instance.
    ///
    /// # Panics
    ///
    /// Panics if no valid search strategy is provided.
    ///
    /// # Examples
    ///
    /// ```
    /// use clap::{Command, Arg};
    /// use seer::search_strategies::search_strategy_factory::SearchStrategyFactory;
    /// use seer::frequency_strategies::frequency_strategy_factory::FrequencyStrategyFactory;
    /// use seer::search_strategies::search_strategy_type::SearchStrategyType;
    ///
    /// let cmd = Command::new("test")
    ///     .arg(Arg::new("exact").long("exact").value_name("EXACT").action(clap::ArgAction::Set));
    /// let matches = cmd.try_get_matches_from(vec!["test", "--exact", "foo"]).unwrap();
    ///
    /// let strategy = SearchStrategyFactory::make(
    ///     &matches,
    ///     FrequencyStrategyFactory::make_for_testing()
    /// );
    /// assert_eq!(strategy.strategy_type(), SearchStrategyType::Exact);
    /// ```
    pub fn make(
        inputs: &ArgMatches,
        frequency_strategy: Box<dyn FrequencyStrategy>,
    ) -> Box<dyn SearchStrategy> {
        if let Some(exact) = inputs.get_one::<String>("exact") {
            return Box::new(ExactSearchStrategy::new(exact.clone(), frequency_strategy));
        } else if let Some(regex) = inputs.get_one::<String>("regex") {
            return Box::new(RegexSearchStrategy::new(regex.clone(), frequency_strategy));
        } else if let Some(between) = inputs.get_many::<String>("between") {
            let mut between_iter = between.into_iter();
            let from = between_iter.next().unwrap().clone();
            let to = between_iter.next().unwrap().clone();
            let exclude_matches = inputs.get_flag("exclude_matches");
            return Box::new(BetweenSearchStrategy::new(
                from,
                to,
                exclude_matches,
                frequency_strategy,
            ));
        }

        panic!("A search strategy must be provided");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        frequency_strategies::frequency_strategy_factory::FrequencyStrategyFactory,
        search_strategies::search_strategy_type::SearchStrategyType,
    };
    use clap::{Arg, Command};

    fn get_command() -> Command {
        Command::new("test")
            .arg(
                Arg::new("exact")
                    .long("exact")
                    .value_name("EXACT")
                    .action(clap::ArgAction::Set),
            )
            .arg(
                Arg::new("regex")
                    .long("regex")
                    .value_name("REGEX")
                    .action(clap::ArgAction::Set),
            )
            .arg(
                Arg::new("between")
                    .long("between")
                    .num_args(2)
                    .value_names(&["begin", "end"]),
            )
            .arg(
                Arg::new("exclude_matches")
                    .long("exclude_matches")
                    .action(clap::ArgAction::SetTrue),
            )
    }

    #[test]
    fn test_exact_search_strategy() {
        let cmd = get_command();
        let matches = cmd
            .try_get_matches_from(vec!["test", "--exact", "foo"])
            .unwrap();
        let strategy =
            SearchStrategyFactory::make(&matches, FrequencyStrategyFactory::make_for_testing());
        assert_eq!(strategy.strategy_type(), SearchStrategyType::Exact);
    }

    #[test]
    fn test_regex_search_strategy() {
        let cmd = get_command();
        let matches = cmd
            .try_get_matches_from(vec!["test", "--regex", "foo"])
            .unwrap();
        let strategy =
            SearchStrategyFactory::make(&matches, FrequencyStrategyFactory::make_for_testing());
        assert_eq!(strategy.strategy_type(), SearchStrategyType::Regex);
    }

    #[test]
    fn test_between_search_strategy() {
        let cmd = get_command();
        let matches = cmd
            .try_get_matches_from(vec!["test", "--between", "foo", "bar"])
            .unwrap();
        let strategy =
            SearchStrategyFactory::make(&matches, FrequencyStrategyFactory::make_for_testing());
        assert_eq!(strategy.strategy_type(), SearchStrategyType::Between);
    }

    #[test]
    #[should_panic(expected = "A search strategy must be provided")]
    fn test_no_search_strategy() {
        let cmd = get_command();
        let matches = cmd.try_get_matches_from(vec!["test"]).unwrap();
        SearchStrategyFactory::make(&matches, FrequencyStrategyFactory::make_for_testing());
    }
}
