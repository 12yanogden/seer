use crate::hit::Hit;
use crate::search_strategies::search_strategy::SearchStrategy;
use regex::Regex;
use std::collections::HashMap;

pub struct BetweenSearchStrategy;

impl SearchStrategy for BetweenSearchStrategy {
    fn search(&self, searchable: &str, params: &HashMap<String, String>) -> Vec<Hit> {
        let mut hits = Vec::new();

        // Extract parameters
        let from_pattern = match params.get("from") {
            Some(pattern) => pattern,
            None => return hits,
        };
        let to_pattern = match params.get("to") {
            Some(pattern) => pattern,
            None => return hits,
        };
        let exclude_matches = match params.get("exclude_matches") {
            Some(value) => value == "true",
            None => false,
        };

        // Compile regex patterns
        let from_regex = match Regex::new(from_pattern) {
            Ok(regex) => regex,
            Err(_) => return hits,
        };
        let to_regex = match Regex::new(to_pattern) {
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
                let hit_value = if exclude_matches {
                    &searchable[from_end..to_start]
                } else {
                    &searchable[from_start..to_end]
                };
                let hit_position = if exclude_matches {
                    from_end
                } else {
                    from_start
                };

                // Add hit to results
                hits.push(Hit {
                    value: hit_value.to_string(),
                    position: hit_position,
                });
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
    use std::collections::HashMap;

    #[test]
    fn test_between_strategy_including_matches() {
        let from = r"start";
        let to = r"end";
        let strategy = BetweenSearchStrategy;
        let mut params = HashMap::new();
        params.insert(String::from("from"), String::from(from));
        params.insert(String::from("to"), String::from(to));
        params.insert(String::from("exclude_matches"), String::from("false"));

        let searchable = "start123endstart456endstart789end";
        let hits = strategy.search(searchable, &params);

        assert_eq!(hits.len(), 3);
        assert_eq!(hits[0].value, "start123end");
        assert_eq!(hits[0].position, 0);
        assert_eq!(hits[0].end_position(), 10);
        assert_eq!(hits[1].value, "start456end");
        assert_eq!(hits[1].position, 11);
        assert_eq!(hits[1].end_position(), 21);
        assert_eq!(hits[2].value, "start789end");
        assert_eq!(hits[2].position, 22);
        assert_eq!(hits[2].end_position(), 32);
    }

    #[test]
    fn test_between_strategy_excluding_matches() {
        let from = r"start";
        let to = r"end";
        let strategy = BetweenSearchStrategy;
        let mut params = HashMap::new();
        params.insert(String::from("from"), String::from(from));
        params.insert(String::from("to"), String::from(to));
        params.insert(String::from("exclude_matches"), String::from("true"));

        let searchable = "start123endstart456endstart789end";
        let hits = strategy.search(searchable, &params);

        assert_eq!(hits.len(), 3);
        assert_eq!(hits[0].value, "123");
        assert_eq!(hits[0].position, 5);
        assert_eq!(hits[0].end_position(), 7);
        assert_eq!(hits[1].value, "456");
        assert_eq!(hits[1].position, 16);
        assert_eq!(hits[1].end_position(), 18);
        assert_eq!(hits[2].value, "789");
        assert_eq!(hits[2].position, 27);
        assert_eq!(hits[2].end_position(), 29);
    }
}
