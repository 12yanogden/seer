use super::handler::Handler;
use crate::dto::dto::DTO;

pub struct EditHandler;

impl EditHandler {
    pub fn new() -> Self {
        Self
    }
}

impl Handler for EditHandler {
    fn handle(&mut self, dto: &mut DTO) {
        // Transform hits into edits based on the operation type
        if let Some(inputs) = dto.get_inputs().as_ref().cloned() {
            // Prepare edits to add in a separate vector
            let mut edits_to_add = Vec::new();
            
            // Process hits and prepare edits
            for hit in dto.get_hits() {
                let new_value = if let Some(append_text) = inputs.get_one::<String>("append") {
                    // Append operation: add text after the hit
                    format!("{}{}", hit.get_value(), append_text)
                } else if let Some(prepend_text) = inputs.get_one::<String>("prepend") {
                    // Prepend operation: add text before the hit
                    format!("{}{}", prepend_text, hit.get_value())
                } else if let Some(replace_text) = inputs.get_one::<String>("replace-with") {
                    // Replace operation: replace the entire hit with new text
                    replace_text.clone()
                } else {
                    // No edit operation specified, keep original value
                    hit.get_value().to_string()
                };
                
                edits_to_add.push((hit.get_position(), new_value));
            }
            
            // Now add all the edits
            for (position, value) in edits_to_add {
                dto.add_edit(position, value);
            }
        }
    }
}
