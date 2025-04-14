use super::hit::Hit;
use super::search_strategy::SearchStrategy;
use super::search_strategy_type::SearchStrategyType;
use crate::frequency_strategies::frequency_strategy::FrequencyStrategy;
use regex::Regex;

/// A search strategy that finds text between two regex patterns.
pub struct BetweenSearchStrategy {
    from: String,
    to: String,
    exclude_matches: bool,
    frequency_strategy: Box<dyn FrequencyStrategy>,
}

impl BetweenSearchStrategy {
    pub fn new(
        from: String,
        to: String,
        exclude_matches: bool,
        frequency_strategy: Box<dyn FrequencyStrategy>,
    ) -> Self {
        Self {
            from,
            to,
            exclude_matches,
            frequency_strategy,
        }
    }
}

impl SearchStrategy for BetweenSearchStrategy {
    fn strategy_type(&self) -> SearchStrategyType {
        SearchStrategyType::Between
    }

    /// Searches for text between two regex patterns.
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
    /// use seer::search_strategies::between_search_strategy::BetweenSearchStrategy;
    /// use seer::search_strategies::search_strategy::SearchStrategy;
    /// use seer::frequency_strategies::frequency_strategy_factory::FrequencyStrategyFactory;
    ///
    /// let mut strategy = BetweenSearchStrategy::new(
    ///     String::from("start"),
    ///     String::from("end"),
    ///     false,
    ///     FrequencyStrategyFactory::make_for_testing(),
    /// );
    /// let searchable = "start123endstart456endstart789end";
    /// let hits = strategy.search(searchable);
    ///
    /// assert_eq!(hits.len(), 3);
    /// assert_eq!(hits[0].get_value(), "start123end");
    /// assert_eq!(hits[0].get_position(), 0);
    /// assert_eq!(hits[0].get_end_position(), 10);
    /// ```
    fn search(&mut self, searchable: &str) -> Vec<Hit> {
        let mut hits = Vec::new();

        // Compile regex patterns
        let from_regex = match Regex::new(&self.from) {
            Ok(regex) => regex,
            Err(_) => return hits,
        };
        let to_regex = match Regex::new(&self.to) {
            Ok(regex) => regex,
            Err(_) => return hits,
        };

        let mut pos = 0;

        // Search for matches
        while let Some(from_match) = from_regex.find(&searchable[pos..]) {
            let from_start = pos + from_match.start();
            let from_end = pos + from_match.end();
            pos = from_end;

            if let Some(to_match) = to_regex.find(&searchable[pos..]) {
                let to_start = pos + to_match.start();
                let to_end = pos + to_match.end();
                pos = to_end;

                // Determine hit value and position
                let hit_value = if self.exclude_matches {
                    &searchable[from_end..to_start]
                } else {
                    &searchable[from_start..to_end]
                };
                let hit_position = if self.exclude_matches {
                    from_end
                } else {
                    from_start
                };

                // Add hit to results
                if self.frequency_strategy.matches_frequency() {
                    hits.push(Hit::new(hit_value.to_string(), hit_position));
                }

                // If the frequency strategy is done, return early
                if self.frequency_strategy.is_done() {
                    return hits;
                }
            } else {
                break;
            }
        }

        hits
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frequency_strategies::frequency_strategy_factory::FrequencyStrategyFactory;

    #[test]
    fn test_between_strategy_including_matches() {
        let from = r"start";
        let to = r"end";
        let mut strategy = BetweenSearchStrategy::new(
            from.to_string(),
            to.to_string(),
            false,
            FrequencyStrategyFactory::make_for_testing(),
        );

        let searchable = "start123endstart456endstart789end";
        let hits = strategy.search(searchable);

        assert_eq!(hits.len(), 3);
        assert_eq!(hits[0].get_value(), "start123end");
        assert_eq!(hits[0].get_position(), 0);
        assert_eq!(hits[0].get_end_position(), 10);
        assert_eq!(hits[1].get_value(), "start456end");
        assert_eq!(hits[1].get_position(), 11);
        assert_eq!(hits[1].get_end_position(), 21);
        assert_eq!(hits[2].get_value(), "start789end");
        assert_eq!(hits[2].get_position(), 22);
        assert_eq!(hits[2].get_end_position(), 32);
    }

    #[test]
    fn test_between_strategy_excluding_matches() {
        let from = r"start";
        let to = r"end";
        let mut strategy = BetweenSearchStrategy::new(
            from.to_string(),
            to.to_string(),
            true,
            FrequencyStrategyFactory::make_for_testing(),
        );

        let searchable = "start123endstart456endstart789end";
        let hits = strategy.search(searchable);

        assert_eq!(hits.len(), 3);
        assert_eq!(hits[0].get_value(), "123");
        assert_eq!(hits[0].get_position(), 5);
        assert_eq!(hits[0].get_end_position(), 7);
        assert_eq!(hits[1].get_value(), "456");
        assert_eq!(hits[1].get_position(), 16);
        assert_eq!(hits[1].get_end_position(), 18);
        assert_eq!(hits[2].get_value(), "789");
        assert_eq!(hits[2].get_position(), 27);
        assert_eq!(hits[2].get_end_position(), 29);
    }
}
