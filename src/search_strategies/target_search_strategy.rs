use super::search_strategy::SearchStrategy;
use super::search_strategy_type::SearchStrategyType;
use crate::frequency_strategies::frequency_strategy::FrequencyStrategy;
use crate::hit::Hit;

/// A search strategy that finds occurrences of a target string.
pub struct TargetSearchStrategy {
    target: String,
    frequency_strategy: Box<dyn FrequencyStrategy>,
}

impl TargetSearchStrategy {
    pub fn new(target: String, frequency_strategy: Box<dyn FrequencyStrategy>) -> Self {
        Self {
            target,
            frequency_strategy,
        }
    }
}

impl SearchStrategy for TargetSearchStrategy {
    fn strategy_type(&self) -> SearchStrategyType {
        SearchStrategyType::Target
    }

    /// Searches for occurrences of a target string.
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
    /// use seek::search_strategies::target_search_strategy::TargetSearchStrategy;
    /// use seek::search_strategies::search_strategy::SearchStrategy;
    /// use seek::frequency_strategies::frequency_strategy_factory::FrequencyStrategyFactory;
    ///
    /// let mut strategy = TargetSearchStrategy::new(
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
        let target_len = self.target.len();
        let mut pos = 0;
        while let Some(start) = searchable[pos..].find(&self.target) {
            let position = pos + start;
            if self.frequency_strategy.matches_frequency() {
                hits.push(Hit::new(self.target.clone(), position));
            }
            if self.frequency_strategy.is_done() {
                return hits;
            }
            pos = position + target_len;
        }
        hits
    }
}

#[cfg(test)]
mod tests {
    use crate::frequency_strategies::frequency_strategy_factory::FrequencyStrategyFactory;

    use super::*;

    #[test]
    fn test_target_strategy() {
        let target = "test";
        let mut strategy = TargetSearchStrategy::new(
            target.to_string(),
            FrequencyStrategyFactory::make_for_testing(),
        );

        let searchable = "test1234567890tester1234567890retest1234567890test";
        let hits = strategy.search(searchable);

        assert_eq!(hits.len(), 4);
        assert_eq!(hits[0].get_value(), target);
        assert_eq!(hits[0].get_position(), 0);
        assert_eq!(hits[0].get_end_position(), 3);
        assert_eq!(hits[1].get_value(), target);
        assert_eq!(hits[1].get_position(), 14);
        assert_eq!(hits[1].get_end_position(), 17);
        assert_eq!(hits[2].get_value(), target);
        assert_eq!(hits[2].get_position(), 32);
        assert_eq!(hits[2].get_end_position(), 35);
        assert_eq!(hits[3].get_value(), target);
        assert_eq!(hits[3].get_position(), 46);
        assert_eq!(hits[3].get_end_position(), 49);
    }
}
