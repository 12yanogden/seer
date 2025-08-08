use super::handler::Handler;
use crate::dto::dto::DTO;
use crate::strategy_factories::source_strategy_factory::SourceStrategyFactory;

pub struct GetSourcesHandler;

impl GetSourcesHandler {
    pub fn new() -> Self {
        Self
    }
}

impl Handler for GetSourcesHandler {
    fn handle(&mut self, dto: &mut DTO) {
        // Clone inputs to avoid borrow checker issues
        if let Some(inputs) = dto.get_inputs().as_ref().cloned() {
            if let Err(e) = SourceStrategyFactory::process_sources(&inputs, dto) {
                eprintln!("Error processing sources: {}", e);
            }
        }
    }
}
