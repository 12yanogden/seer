use crate::dto::dto::DTO;
use crate::strategies::traits::strategy::Strategy;
use crate::strategies::enums::strategy_type::StrategyType;

/// A strategy for processing text input provided as a command-line argument.
/// 
/// This strategy reads a string input specified by the `--text` command line argument
/// and adds it as a source to the DTO. The source name is set to "text".
/// 
/// # Examples
/// 
/// ```rust
/// use seer::strategies::text_strategy::TextStrategy;
/// use seer::strategies::traits::strategy::Strategy;
/// use seer::dto::dto::DTO;
/// use seer::arg_buliders::text_arg_builder::TextArgBuilder;
/// use seer::handlers::build_command_handler::CommandBuilder;
/// use clap::Command;
/// 
/// // Create a DTO
/// let mut dto = DTO::new("seer", "1.0.0", "author", "A search tool");
/// 
/// // Create command with text argument using TextArgBuilder
/// let mut app = Command::new("test");
/// TextArgBuilder::build(&mut app);
/// 
/// let matches = app.try_get_matches_from(vec!["test", "--text", "example text"]).unwrap();
/// dto.set_inputs(matches);
/// 
/// // Create and run the strategy
/// let strategy = TextStrategy::new();
/// strategy.run(&mut dto);
/// 
/// // The text should now be added as a source
/// assert_eq!(dto.get_source_count(), 1);
/// ```
pub struct TextStrategy;

impl TextStrategy {
    /// Creates a new TextStrategy instance.
    pub fn new() -> Self {
        Self
    }
}

impl Strategy for TextStrategy {
    /// Returns the strategy type identifier.
    fn strategy_type(&self) -> StrategyType {
        StrategyType::Text
    }

    /// Executes the text reading strategy.
    /// 
    /// This method:
    /// 1. Extracts the text from the `--text` argument in the DTO's inputs
    /// 2. Adds the text as a source to the DTO with "text" as the source name
    /// 
    /// If no `--text` argument is present, the method returns
    /// without adding a source, allowing the application to continue.
    fn run(&self, dto: &mut DTO) {
        // Attempt to get the text from command line arguments
        if let Some(text) = dto.get_input::<String>("text").cloned() {
            // Add the text as a source to the DTO
            dto.add_source("text", &text);
        }
        // If no text argument is present, do nothing
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arg_buliders::text_arg_builder::TextArgBuilder;
    use crate::handlers::build_command_handler::CommandBuilder;
    use clap::Command;

    #[test]
    fn test_strategy_type() {
        let strategy = TextStrategy::new();
        assert_eq!(strategy.strategy_type(), StrategyType::Text);
    }

    #[test]
    fn test_run_with_text_input() {
        // Create DTO and command
        let mut dto = DTO::new("seer", "1.0.0", "author", "A search tool");
        let mut cmd = Command::new("test_command");
        TextArgBuilder::build(&mut cmd);
        
        let matches = cmd.try_get_matches_from(vec!["test", "--text", "example text"]).unwrap();
        dto.set_inputs(matches);

        // Run the strategy
        let strategy = TextStrategy::new();
        strategy.run(&mut dto);

        // Verify the source was added
        assert_eq!(dto.get_source_count(), 1);
        let sources = dto.get_sources();
        assert_eq!(sources[0].get_name(), "text");
        assert_eq!(sources[0].get_text(), "example text");
    }
}
