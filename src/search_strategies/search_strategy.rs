use crate::hit::Hit;

pub trait SearchStrategy {
    fn search(&self, searchable: &str) -> Vec<Hit>;
}
