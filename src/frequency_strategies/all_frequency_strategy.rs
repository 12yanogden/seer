use super::frequency_strategy::FrequencyStrategy;
use super::frequency_strategy_type::FrequencyStrategyType;

pub struct AllFrequencyStrategy {}

impl AllFrequencyStrategy {
    pub fn new() -> Self {
        Self {}
    }
}

impl FrequencyStrategy for AllFrequencyStrategy {
    fn strategy_type(&self) -> FrequencyStrategyType {
        FrequencyStrategyType::All
    }

    fn matches_frequency(&mut self) -> bool {
        true
    }

    fn is_done(&self) -> bool {
        false
    }
}
