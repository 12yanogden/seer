use super::hit::Hit;
use super::search_strategy::SearchStrategy;
use super::search_strategy_type::SearchStrategyType;
use crate::frequency_strategies::frequency_strategy::FrequencyStrategy;
use regex::Regex;

/// A search strategy that finds text matching a regex pattern.
pub struct RegexSearchStrategy {
    regex: String,
    frequency_strategy: Box<dyn FrequencyStrategy>,
}

impl RegexSearchStrategy {
    pub fn new(regex: String, frequency_strategy: Box<dyn FrequencyStrategy>) -> Self {
        Self {
            regex,
            frequency_strategy,
        }
    }
}

impl SearchStrategy for RegexSearchStrategy {
    fn strategy_type(&self) -> SearchStrategyType {
        SearchStrategyType::Regex
    }

    /// Searches for text matching a regex pattern.
    ///
    /// # Parameters
    /// - `searchable`: The text to search within.
    ///
    /// # Returns
    /// A vector of `Hit` structs representing the matches found.
    ///
    /// # Examples
    ///
    /// ```
    /// use seer::search_strategies::regex_search_strategy::RegexSearchStrategy;
    /// use seer::search_strategies::search_strategy::SearchStrategy;
    /// use seer::frequency_strategies::frequency_strategy_factory::FrequencyStrategyFactory;
    ///
    /// let mut strategy = RegexSearchStrategy::new(
    ///     String::from(r"\d+"),
    ///     FrequencyStrategyFactory::make_for_testing()
    /// );
    ///
    /// let searchable = "test1234567890tester1234567890retest1234567890test";
    /// let hits = strategy.search(searchable);
    ///
    /// assert_eq!(hits.len(), 3);
    /// assert_eq!(hits[0].get_value(), "1234567890");
    /// assert_eq!(hits[0].get_position(), 4);
    /// ```
    fn search(&mut self, searchable: &str) -> Vec<Hit> {
        let mut hits = Vec::new();
        if let Ok(regex) = Regex::new(&self.regex) {
            for mat in regex.find_iter(searchable) {
                if self.frequency_strategy.matches_frequency() {
                    hits.push(Hit::new(mat.as_str().to_string(), mat.start()));
                }

                if self.frequency_strategy.is_done() {
                    return hits;
                }
            }
        }
        hits
    }
}

#[cfg(test)]
mod tests {
    use crate::frequency_strategies::frequency_strategy_factory::FrequencyStrategyFactory;

    use super::*;

    #[test]
    fn test_regex_strategy() {
        let regex = r"[a-z]*test[a-z]*";
        let mut strategy = RegexSearchStrategy::new(
            String::from(regex),
            FrequencyStrategyFactory::make_for_testing(),
        );

        let searchable = "test1234567890tester1234567890retest1234567890test";
        let hits = strategy.search(searchable);

        assert_eq!(hits.len(), 4);
        assert_eq!(hits[0].get_value(), "test");
        assert_eq!(hits[0].get_position(), 0);
        assert_eq!(hits[0].get_end_position(), 3);
        assert_eq!(hits[1].get_value(), "tester");
        assert_eq!(hits[1].get_position(), 14);
        assert_eq!(hits[1].get_end_position(), 19);
        assert_eq!(hits[2].get_value(), "retest");
        assert_eq!(hits[2].get_position(), 30);
        assert_eq!(hits[2].get_end_position(), 35);
        assert_eq!(hits[3].get_value(), "test");
        assert_eq!(hits[3].get_position(), 46);
        assert_eq!(hits[3].get_end_position(), 49);
    }
}
