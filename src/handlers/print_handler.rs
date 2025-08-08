use super::handler::Handler;
use crate::dto::dto::DTO;
use crate::output_strategies::traits::output_strategy::OutputStrategy;
use crate::output_strategies::stdout_strategy::StdoutStrategy;
use crate::output_strategies::edit_in_place_strategy::EditInPlaceStrategy;

pub struct PrintHandler;

impl PrintHandler {
    pub fn new() -> Self {
        Self
    }
}

impl Handler for PrintHandler {
    fn handle(&mut self, dto: &mut DTO) {
        // Determine which output strategy to use based on arguments
        if let Some(inputs) = dto.get_inputs() {
            if inputs.get_flag("edit-in-place") {
                let strategy = EditInPlaceStrategy::new();
                if let Err(e) = strategy.output(dto) {
                    eprintln!("Error applying edit-in-place strategy: {}", e);
                }
            } else {
                // Default to stdout
                let strategy = StdoutStrategy::new();
                if let Err(e) = strategy.output(dto) {
                    eprintln!("Error applying stdout strategy: {}", e);
                }
            }
        } else {
            // Fallback to stdout if no inputs available
            let strategy = StdoutStrategy::new();
            if let Err(e) = strategy.output(dto) {
                eprintln!("Error applying stdout strategy: {}", e);
            }
        }
    }
}
