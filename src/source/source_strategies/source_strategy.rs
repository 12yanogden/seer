use super::source::Source;
use super::source_strategy_type::SourceStrategyType;

pub trait SourceStrategy {
    fn strategy_type(&self) -> SourceStrategyType;
    fn get_sources() -> Vec<Source>;
}