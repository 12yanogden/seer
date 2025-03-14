use crate::hit::Hit;
use crate::search_strategies::search_strategy::SearchStrategy;
use regex::Regex;

/// A search strategy that finds text between two regex patterns.
pub struct BetweenSearchStrategy {
    from: String,
    to: String,
    exclude_matches: bool,
}

impl BetweenSearchStrategy {
    pub fn new(from: String, to: String, exclude_matches: bool) -> Self {
        Self {
            from,
            to,
            exclude_matches,
        }
    }
}

impl SearchStrategy for BetweenSearchStrategy {
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
    /// use seek::search_strategies::between::BetweenSearchStrategy;
    /// use seek::search_strategies::search_strategy::SearchStrategy;
    ///
    /// let strategy = BetweenSearchStrategy::new(String::from("start"), String::from("end"), false);
    /// let searchable = "start123endstart456endstart789end";
    /// let hits = strategy.search(searchable);
    ///
    /// assert_eq!(hits.len(), 3);
    /// assert_eq!(hits[0].get_value(), "start123end");
    /// assert_eq!(hits[0].get_position(), 0);
    /// assert_eq!(hits[0].get_end_position(), 10);
    /// ```
    fn search(&self, searchable: &str) -> Vec<Hit> {
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
                hits.push(Hit::new(hit_value.to_string(), hit_position));
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

    #[test]
    fn test_between_strategy_including_matches() {
        let from = r"start";
        let to = r"end";
        let strategy = BetweenSearchStrategy::new(from.to_string(), to.to_string(), false);

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
        let strategy = BetweenSearchStrategy::new(from.to_string(), to.to_string(), true);

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
