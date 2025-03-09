use crate::hit::Hit;
use crate::search_strategies::search_strategy::SearchStrategy;
use regex::Regex;
use std::collections::HashMap;

pub struct RegexSearchStrategy;

impl SearchStrategy for RegexSearchStrategy {
    fn search(&self, searchable: &str, params: &HashMap<String, String>) -> Vec<Hit> {
        let mut hits = Vec::new();
        if let Some(regex) = params.get("regex") {
            if let Ok(regex) = Regex::new(regex) {
                for mat in regex.find_iter(searchable) {
                    hits.push(Hit {
                        value: mat.as_str().to_string(),
                        position: mat.start(),
                    });
                }
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
    fn test_regex_strategy() {
        let regex = r"[a-z]*test[a-z]*";
        let strategy = RegexSearchStrategy;
        let mut params = HashMap::new();
        params.insert(String::from("regex"), String::from(regex));

        let searchable = "test1234567890tester1234567890retest1234567890test";
        let hits = strategy.search(searchable, &params);

        assert_eq!(hits.len(), 4);
        assert_eq!(hits[0].value, "test");
        assert_eq!(hits[0].position, 0);
        assert_eq!(hits[0].end_position(), 3);
        assert_eq!(hits[1].value, "tester");
        assert_eq!(hits[1].position, 14);
        assert_eq!(hits[1].end_position(), 19);
        assert_eq!(hits[2].value, "retest");
        assert_eq!(hits[2].position, 30);
        assert_eq!(hits[2].end_position(), 35);
        assert_eq!(hits[3].value, "test");
        assert_eq!(hits[3].position, 46);
        assert_eq!(hits[3].end_position(), 49);
    }
}
