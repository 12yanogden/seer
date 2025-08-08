use super::traits::output_strategy::OutputStrategy;
use crate::dto::dto::DTO;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Output strategy that edits files in place
pub struct EditInPlaceStrategy;

impl EditInPlaceStrategy {
    pub fn new() -> Self {
        Self
    }
    
    /// Apply edits to a source text
    fn apply_edits_to_text(&self, text: &str, edits: &[(usize, &str)]) -> String {
        if edits.is_empty() {
            return text.to_string();
        }
        
        // Sort edits by position in reverse order to avoid position shifts
        let mut sorted_edits = edits.to_vec();
        sorted_edits.sort_by(|a, b| b.0.cmp(&a.0));
        
        let mut result = text.to_string();
        
        for (position, new_value) in sorted_edits {
            if position <= result.len() {
                // Find the end of the original hit by looking for the next character boundary
                // This is a simplified approach - in a real implementation, we'd need to track
                // the original hit length from the DTO
                result.replace_range(position..position, new_value);
            }
        }
        
        result
    }
}

impl OutputStrategy for EditInPlaceStrategy {
    fn output(&self, dto: &DTO) -> Result<(), Box<dyn std::error::Error>> {
        let edits = dto.get_edits();
        
        if edits.is_empty() {
            return Ok(()); // Nothing to edit
        }
        
        // Group edits by source file
        let mut edits_by_source: HashMap<String, Vec<(usize, &str)>> = HashMap::new();
        
        // Get source indices for each edit (simplified approach)
        for (_edit_index, edit) in edits.iter().enumerate() {
            // For now, we'll apply all edits to all file sources
            // In a complete implementation, we'd track which source each edit belongs to
            for source in dto.get_sources() {
                let source_name = source.get_name().to_string();
                if !source_name.starts_with("pipe") && !source_name.starts_with("text") {
                    // Only apply to file sources, not pipes or text inputs
                    edits_by_source
                        .entry(source_name)
                        .or_insert_with(Vec::new)
                        .push((edit.get_position(), edit.get_new_value()));
                }
            }
        }
        
        // Apply edits to each file
        for (file_path, file_edits) in edits_by_source {
            if Path::new(&file_path).exists() {
                let original_content = fs::read_to_string(&file_path)?;
                let modified_content = self.apply_edits_to_text(&original_content, &file_edits);
                fs::write(&file_path, modified_content)?;
                eprintln!("Modified: {}", file_path);
            } else {
                eprintln!("Warning: File not found: {}", file_path);
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dto::dto::DTO;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_edit_in_place_strategy() {
        // Create a temporary file
        let temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path().to_str().unwrap();
        fs::write(file_path, "original content").unwrap();
        
        let mut dto = DTO::new("test", "1.0", "author", "description");
        dto.add_source(file_path, "original content");
        dto.add_edit(0, "modified ".to_string());
        
        let strategy = EditInPlaceStrategy::new();
        let result = strategy.output(&dto);
        
        assert!(result.is_ok());
    }

    #[test]
    fn test_apply_edits_to_text() {
        let strategy = EditInPlaceStrategy::new();
        let text = "Hello world";
        let edits = vec![(6, "beautiful ")];
        
        let result = strategy.apply_edits_to_text(text, &edits);
        assert_eq!(result, "Hello beautiful world");
    }
}
