use crate::hit::Hit; // Rename imported Hit
use crate::search_strategies::search_strategy::SearchStrategy;
use std::collections::HashMap;

pub struct TargetSearchStrategy;

impl SearchStrategy for TargetSearchStrategy {
    fn search(&self, searchable: &str, params: &HashMap<String, String>) -> Vec<Hit> {
        let mut hits = Vec::new();
        if let Some(target) = params.get("target") {
            let target_len = target.len();
            let mut pos = 0;
            while let Some(start) = searchable[pos..].find(target) {
                let position = pos + start;
                hits.push(Hit {
                    value: target.clone(),
                    position,
                });
                pos = position + target_len;
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
    fn test_target_strategy() {
        let target = "test";
        let strategy = TargetSearchStrategy;
        let mut params = HashMap::new();
        params.insert(String::from("target"), String::from(target));

        let searchable = "test1234567890tester1234567890retest1234567890test";
        let hits = strategy.search(searchable, &params);

        assert_eq!(hits.len(), 4);
        assert_eq!(hits[0].value, target);
        assert_eq!(hits[0].position, 0);
        assert_eq!(hits[0].end_position(), 3);
        assert_eq!(hits[1].value, target);
        assert_eq!(hits[1].position, 14);
        assert_eq!(hits[1].end_position(), 17);
        assert_eq!(hits[2].value, target);
        assert_eq!(hits[2].position, 32);
        assert_eq!(hits[2].end_position(), 35);
        assert_eq!(hits[3].value, target);
        assert_eq!(hits[3].position, 46);
        assert_eq!(hits[3].end_position(), 49);
    }
}
