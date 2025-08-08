use crate::dto::dto::DTO;
use crate::strategies::traits::strategy::Strategy;
use crate::strategies::enums::strategy_type::StrategyType;
use std::io::{self, Read};

/// A strategy for processing piped input from stdin.
/// 
/// This strategy reads content from stdin (pipe input) and adds it as a source to the DTO.
/// The source name is set to "pipe" to indicate the input came from a pipe.
/// 
/// # Examples
/// 
/// ```rust
/// use seer::strategies::pipe_strategy::PipeStrategy;
/// use seer::strategies::traits::strategy::Strategy;
/// use seer::dto::dto::DTO;
/// 
/// // Create a DTO
/// let mut dto = DTO::new("seer", "1.0.0", "author", "A search tool");
/// 
/// // Create and run the strategy (will attempt to read from stdin)
/// let strategy = PipeStrategy::new();
/// strategy.run(&mut dto);
/// 
/// // If there was piped input, it will be added as a source
/// // The source count will be 1 if pipe input was available, 0 otherwise
/// ```
/// 
/// # Pipe Reading
/// 
/// The strategy attempts to read from stdin using `stdin().read_to_string()`. If no input
/// is available or the input is empty, no source is added to the DTO. This allows the
/// strategy to be safely called even when no pipe input is provided.
pub struct PipeStrategy;

impl PipeStrategy {
    /// Creates a new PipeStrategy instance.
    pub fn new() -> Self {
        Self
    }

    /// Reads piped input from stdin.
    ///
    /// # Returns
    ///
    /// An `Option<String>` containing the piped input if available.
    fn read_pipe() -> Option<String> {
        let mut pipe = String::new();
        let could_read_input = io::stdin().read_to_string(&mut pipe).is_ok();

        if could_read_input && !pipe.is_empty() {
            Some(pipe)
        } else {
            None
        }
    }
}

impl Strategy for PipeStrategy {
    /// Returns the strategy type identifier.
    fn strategy_type(&self) -> StrategyType {
        StrategyType::Pipe
    }

    /// Executes the pipe reading strategy.
    /// 
    /// This method:
    /// 1. Attempts to read input from stdin
    /// 2. If input is available and non-empty, adds it as a source to the DTO
    /// 3. Uses "pipe" as the source name to indicate the input came from a pipe
    /// 
    /// If no pipe input is available or the input is empty, the method returns
    /// without adding a source, allowing the application to continue.
    fn run(&self, dto: &mut DTO) {
        // Attempt to read from pipe
        if let Some(pipe_content) = Self::read_pipe() {
            // Add the pipe content as a source to the DTO
            dto.add_source("pipe", &pipe_content);
        }
        // If no pipe input is available, do nothing
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static::lazy_static! {
        static ref BASE_DTO: DTO = {
            DTO::new("seer", "1.0.0", "author", "A search tool")
        };
    }

    #[test]
    fn test_strategy_type() {
        let strategy = PipeStrategy::new();
        assert_eq!(strategy.strategy_type(), StrategyType::Pipe);
    }

    #[test]
    fn test_run_without_pipe_input() {
        let mut dto = BASE_DTO.clone();
        
        let strategy = PipeStrategy::new();
        strategy.run(&mut dto);

        // Should not add any sources when no pipe input is available
        // Note: This test assumes no pipe input is provided during test execution
        assert_eq!(dto.get_source_count(), 0);
    }

    #[test]
    fn test_new_creates_instance() {
        let strategy = PipeStrategy::new();
        // Just verify we can create an instance
        assert_eq!(strategy.strategy_type(), StrategyType::Pipe);
    }

    // Note: Testing with actual pipe input is challenging in unit tests
    // since it would require setting up stdin input. Integration tests
    // or end-to-end tests would be more appropriate for testing the
    // actual pipe reading functionality.
}
