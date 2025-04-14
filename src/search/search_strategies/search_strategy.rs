use super::hit::Hit;
use super::search_strategy_type::SearchStrategyType;

pub trait SearchStrategy {
    fn strategy_type(&self) -> SearchStrategyType;
    fn search(&mut self, searchable: &str) -> Vec<Hit>;
}
