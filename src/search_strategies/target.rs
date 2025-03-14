use crate::hit::Hit;
use crate::search_strategies::search_strategy::SearchStrategy;
use std::collections::HashMap;

/// A search strategy that finds occurrences of a target string.
pub struct TargetSearchStrategy {
    target: String,
}

impl TargetSearchStrategy {
    pub fn new(target: String) -> Self {
        Self { target }
    }
}

impl SearchStrategy for TargetSearchStrategy {
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
    /// use seek::search_strategies::target::TargetSearchStrategy;
    /// use seek::search_strategies::search_strategy::SearchStrategy;
    ///
    /// let strategy = TargetSearchStrategy::new(String::from("test"));
    ///
    /// let searchable = "test1234567890tester1234567890retest1234567890test";
    /// let hits = strategy.search(searchable);
    ///
    /// assert_eq!(hits.len(), 4);
    /// assert_eq!(hits[0].get_value(), "test");
    /// assert_eq!(hits[0].get_position(), 0);
    /// ```
    fn search(&self, searchable: &str) -> Vec<Hit> {
        let mut hits = Vec::new();
        let target_len = self.target.len();
        let mut pos = 0;
        while let Some(start) = searchable[pos..].find(&self.target) {
            let position = pos + start;
            hits.push(Hit::new(self.target.clone(), position));
            pos = position + target_len;
        }
        hits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_strategy() {
        let target = "test";
        let strategy = TargetSearchStrategy::new(target.to_string());

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
