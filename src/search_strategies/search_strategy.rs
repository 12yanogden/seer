use crate::hit::Hit;
use std::collections::HashMap;

pub trait SearchStrategy {
    fn search(&self, searchable: &str, params: &HashMap<String, String>) -> Vec<Hit>;
}