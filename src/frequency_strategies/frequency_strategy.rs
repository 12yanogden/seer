use super::frequency_strategy_type::FrequencyStrategyType;

pub trait FrequencyStrategy {
    fn strategy_type(&self) -> FrequencyStrategyType;
    fn matches_frequency(&mut self) -> bool;
    fn is_done(&self) -> bool;
}
