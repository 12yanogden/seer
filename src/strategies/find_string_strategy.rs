use super::hit::Hit;
use super::search_strategy::SearchStrategy;
use super::search_strategy_type::SearchStrategyType;
use crate::frequency_strategies::frequency_strategy::FrequencyStrategy;

/// A search strategy that finds occurrences of a exact string.
pub struct ExactSearchStrategy {
    exact: String,
    frequency_strategy: Box<dyn FrequencyStrategy>,
}

impl ExactSearchStrategy {
    pub fn new(exact: String, frequency_strategy: Box<dyn FrequencyStrategy>) -> Self {
        Self {
            exact,
            frequency_strategy,
        }
    }
}

impl SearchStrategy for ExactSearchStrategy {
    fn strategy_type(&self) -> SearchStrategyType {
        SearchStrategyType::Exact
    }

    /// Searches for occurrences of a exact string.
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
    /// use seer::search_strategies::exact_search_strategy::ExactSearchStrategy;
    /// use seer::search_strategies::search_strategy::SearchStrategy;
    /// use seer::frequency_strategies::frequency_strategy_factory::FrequencyStrategyFactory;
    ///
    /// let mut strategy = ExactSearchStrategy::new(
    ///     String::from("test"),
    ///     FrequencyStrategyFactory::make_for_testing()
    /// );
    ///
    /// let searchable = "test1234567890tester1234567890retest1234567890test";
    /// let hits = strategy.search(searchable);
    ///
    /// assert_eq!(hits.len(), 4);
    /// assert_eq!(hits[0].get_value(), "test");
    /// assert_eq!(hits[0].get_position(), 0);
    /// ```
    fn search(&mut self, searchable: &str) -> Vec<Hit> {
        let mut hits = Vec::new();
        let exact_len = self.exact.len();
        let mut pos = 0;
        while let Some(start) = searchable[pos..].find(&self.exact) {
            let position = pos + start;
            if self.frequency_strategy.matches_frequency() {
                hits.push(Hit::new(self.exact.clone(), position));
            }
            if self.frequency_strategy.is_done() {
                return hits;
            }
            pos = position + exact_len;
        }
        hits
    }
}

#[cfg(test)]
mod tests {
    use crate::frequency_strategies::frequency_strategy_factory::FrequencyStrategyFactory;

    use super::*;

    #[test]
    fn test_exact_strategy() {
        let exact = "test";
        let mut strategy = ExactSearchStrategy::new(
            exact.to_string(),
            FrequencyStrategyFactory::make_for_testing(),
        );

        let searchable = "test1234567890tester1234567890retest1234567890test";
        let hits = strategy.search(searchable);

        assert_eq!(hits.len(), 4);
        assert_eq!(hits[0].get_value(), exact);
        assert_eq!(hits[0].get_position(), 0);
        assert_eq!(hits[0].get_end_position(), 3);
        assert_eq!(hits[1].get_value(), exact);
        assert_eq!(hits[1].get_position(), 14);
        assert_eq!(hits[1].get_end_position(), 17);
        assert_eq!(hits[2].get_value(), exact);
        assert_eq!(hits[2].get_position(), 32);
        assert_eq!(hits[2].get_end_position(), 35);
        assert_eq!(hits[3].get_value(), exact);
        assert_eq!(hits[3].get_position(), 46);
        assert_eq!(hits[3].get_end_position(), 49);
    }
}
