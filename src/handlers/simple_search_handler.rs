use super::handler::Handler;
use crate::dto::dto::DTO;

pub struct SimpleSearchHandler;

impl SimpleSearchHandler {
    pub fn new() -> Self {
        Self
    }
}

impl Handler for SimpleSearchHandler {
    fn handle(&mut self, dto: &mut DTO) {
        if let Some(inputs) = dto.get_inputs().as_ref().cloned() {
            // Look for find-string argument
            if let Some(find_string) = inputs.get_one::<String>("find-string") {
                self.search_string(dto, find_string);
            } else if let Some(find_exact) = inputs.get_one::<String>("find-exact") {
                self.search_string(dto, find_exact);
            }
        }
    }
}

impl SimpleSearchHandler {
    fn search_string(&self, dto: &mut DTO, pattern: &str) {
        let mut hits_to_add = Vec::new();
        
        for (source_index, source) in dto.get_sources().iter().enumerate() {
            let text = source.get_text();
            let mut start = 0;
            
            while let Some(pos) = text[start..].find(pattern) {
                let absolute_pos = start + pos;
                hits_to_add.push((pattern.to_string(), absolute_pos, source_index));
                start = absolute_pos + 1; // Move past this match
            }
        }
        
        // Add all the hits
        for (value, position, source_index) in hits_to_add {
            dto.add_hit(value, position, source_index);
        }
    }
}
