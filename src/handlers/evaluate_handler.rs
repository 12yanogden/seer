use super::handler::Handler;
use crate::dto::dto::DTO;

pub struct EvaluateHandler;

impl EvaluateHandler {
    pub fn new() -> Self {
        Self
    }
}

impl Handler for EvaluateHandler {
    fn handle(&mut self, dto: &mut DTO) {
        // Generate evaluation results based on the operation type
        if let Some(inputs) = dto.get_inputs() {
            if inputs.contains_id("count") {
                let count = dto.get_hit_count();
                let evaluation_text = count.to_string();
                // Store evaluation result - for now just print it
                println!("{}", evaluation_text);
            } else if inputs.contains_id("count-by-source") {
                // Count hits grouped by source
                let mut source_counts = std::collections::HashMap::new();
                
                for (_source_index, source) in dto.get_sources().iter().enumerate() {
                    let hit_count = source.get_hit_indices().len();
                    source_counts.insert(source.get_name().to_string(), hit_count);
                }
                
                for (source_name, count) in source_counts {
                    println!("{}: {}", source_name, count);
                }
            }
        }
    }
}
