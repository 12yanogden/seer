use super::enums::strategy_type::StrategyType;
use super::traits::frequency_strategy::FrequencyStrategy;
use super::traits::strategy::Strategy;
use crate::dto::dto::DTO;

/// AllStrategy struct implements the Strategy and FrequencyStrategy traits.
///
/// # Example
/// ```
/// let strategy = AllStrategy::new();
/// ```
pub struct AllStrategy {}

impl AllStrategy {
    /// Creates a new AllStrategy instance.
    ///
    /// # Example
    /// ```
    /// let strategy = AllStrategy::new();
    /// ```
    pub fn new() -> Self {
        Self {}
    }
}

impl Strategy for AllStrategy {
    /// Returns the type of the strategy.
    ///
    /// # Example
    /// ```
    /// let strategy = AllStrategy::new();
    /// assert_eq!(strategy.strategy_type(), StrategyType::All);
    /// ```
    fn strategy_type(&self) -> StrategyType {
        StrategyType::All
    }

    /// Runs the strategy on the given DTO.
    ///
    /// # Example
    /// ```
    /// let mut dto = DTO::new("cmd", "1.0", "author", "about");
    /// dto.add_hit(Hit::new("hit1", 0));
    /// dto.add_hit(Hit::new("hit2", 1));
    /// let strategy = AllStrategy::new();
    /// strategy.run(&mut dto);
    /// // Validate all hits are included
    /// let hits = dto.get_hits();
    /// assert_eq!(hits.len(), 2);
    /// ```
    fn run(&self, dto: &mut DTO) {
        dto
    }
}

impl FrequencyStrategy for AllStrategy {
    /// Returns true if the frequency matches.
    /// For the AllStrategy, the function always returns true.
    /// Every hit is considered a match.
    ///
    /// # Example
    /// ```
    /// let mut strategy = AllStrategy::new();
    /// assert!(strategy.matches_frequency());
    /// ```
    fn matches_frequency(&mut self) -> bool {
        true
    }

    /// Returns false if the strategy is not done.
    /// For the AllStrategy, the function always returns false.
    /// The frequeny will never be met before the search finishes.
    ///
    /// # Example
    /// ```
    /// let strategy = AllStrategy::new();
    /// assert!(!strategy.is_done());
    /// ```
    fn is_done(&self) -> bool {
        false
    }
}
