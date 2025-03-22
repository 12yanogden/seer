use super::frequency_strategy::FrequencyStrategy;
use super::frequency_strategy_type::FrequencyStrategyType;

pub struct EveryNthFrequencyStrategy {
    frequency: usize,
    counter: usize,
    offset: usize,
}

impl EveryNthFrequencyStrategy {
    pub fn new(frequency: usize, offset: usize) -> Self {
        Self {
            frequency: frequency,
            counter: 0,
            offset: offset,
        }
    }

    fn index(&self) -> usize {
        self.counter - 1
    }

    fn increment_counter(&mut self) {
        self.counter += 1;
    }
}

impl FrequencyStrategy for EveryNthFrequencyStrategy {
    fn strategy_type(&self) -> FrequencyStrategyType {
        FrequencyStrategyType::EveryNth
    }

    fn matches_frequency(&mut self) -> bool {
        self.increment_counter();
        let has_reached_offset = self.index() <= self.offset;
        let matches_frequency = ((self.index() - self.offset) % self.frequency) == 0;

        has_reached_offset && matches_frequency
    }

    fn is_done(&self) -> bool {
        false
    }
}
