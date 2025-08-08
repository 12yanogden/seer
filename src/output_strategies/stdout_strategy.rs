use super::traits::output_strategy::OutputStrategy;
use crate::dto::dto::DTO;

/// Output strategy that prints results to stdout
pub struct StdoutStrategy;

impl StdoutStrategy {
    pub fn new() -> Self {
        Self
    }
}

impl OutputStrategy for StdoutStrategy {
    fn output(&self, dto: &DTO) -> Result<(), Box<dyn std::error::Error>> {
        let edits = dto.get_edits();
        let hits = dto.get_hits();

        if !edits.is_empty() {
            // Output edited text
            for edit in edits {
                println!("{}", edit.get_new_value());
            }
        } else if !hits.is_empty() {
            // Output hits if no edits were made
            for hit in hits {
                println!("{}", hit.get_value());
            }
        } else {
            // Output original sources if no hits or edits
            for source in dto.get_sources() {
                println!("{}", source.get_text());
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dto::dto::DTO;

    #[test]
    fn test_stdout_strategy_with_edits() {
        let mut dto = DTO::new("test", "1.0", "author", "description");
        dto.add_source("test_source", "test content");
        dto.add_edit(0, "modified content".to_string());

        let strategy = StdoutStrategy::new();
        let result = strategy.output(&dto);

        assert!(result.is_ok());
    }

    #[test]
    fn test_stdout_strategy_with_hits() {
        let mut dto = DTO::new("test", "1.0", "author", "description");
        dto.add_source("test_source", "test content");
        dto.add_hit("test".to_string(), 0, 0);

        let strategy = StdoutStrategy::new();
        let result = strategy.output(&dto);

        assert!(result.is_ok());
    }
}
