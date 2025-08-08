use seer::dto::dto::DTO;
use seer::dto::hit::Hit;
use seer::dto::edit::Edit;
use seer::output_strategies::traits::output_strategy::OutputStrategy;
use seer::output_strategies::stdout_strategy::StdoutStrategy;
use seer::output_strategies::edit_in_place_strategy::EditInPlaceStrategy;
use tempfile::NamedTempFile;
use std::fs;
use std::io::Write;

#[test]
fn test_dto_complete_pipeline_stdout() {
    let mut dto = DTO::new("test", "1.0", "author", "description");
    
    // Phase 1: Add sources
    dto.add_source("test_source", "Hello world! This is a test string.");
    
    // Phase 2: Add hits (simulating search results)
    dto.add_hit("world".to_string(), 6, 0); // Found "world" at position 6
    dto.add_hit("test".to_string(), 19, 0); // Found "test" at position 19
    
    // Phase 3: Transform hits into edits (simulating append operation)
    dto.add_edit(6, "wonderful world".to_string()); // Replace "world" with "wonderful world"
    dto.add_edit(19, "testing".to_string()); // Replace "test" with "testing"
    
    // Phase 4: Output to stdout
    let strategy = StdoutStrategy::new();
    let result = strategy.output(&dto);
    
    assert!(result.is_ok());
    assert_eq!(dto.get_hit_count(), 2);
    assert_eq!(dto.get_edit_count(), 2);
}

#[test]
fn test_dto_edit_in_place_strategy() {
    // Create a temporary file
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "Hello world! This is a test.").unwrap();
    let file_path = temp_file.path().to_str().unwrap().to_string();
    
    let mut dto = DTO::new("test", "1.0", "author", "description");
    dto.add_source(&file_path, "Hello world! This is a test.");
    dto.add_edit(6, "beautiful world".to_string());
    
    let strategy = EditInPlaceStrategy::new();
    let result = strategy.output(&dto);
    
    assert!(result.is_ok());
}

#[test]
fn test_hit_creation_and_properties() {
    let hit = Hit::new("test_pattern".to_string(), 42);
    
    assert_eq!(hit.get_value(), "test_pattern");
    assert_eq!(hit.get_position(), 42);
    assert_eq!(hit.get_length(), 12); // "test_pattern".len()
    assert_eq!(hit.get_end_position(), 53); // 42 + 12 - 1
}

#[test]
fn test_edit_creation_and_properties() {
    let edit = Edit::new(10, "replacement_text".to_string());
    
    assert_eq!(edit.get_position(), 10);
    assert_eq!(edit.get_new_value(), "replacement_text");
}

#[test]
fn test_dto_add_source_and_hits() {
    let mut dto = DTO::new("test", "1.0", "author", "description");
    
    dto.add_source("source1", "This is the first source");
    dto.add_source("source2", "This is the second source");
    
    assert_eq!(dto.get_source_count(), 2);
    
    dto.add_hit("first".to_string(), 12, 0);
    dto.add_hit("second".to_string(), 12, 1);
    
    assert_eq!(dto.get_hit_count(), 2);
    
    // Check that hits are correctly associated with sources
    let sources = dto.get_sources();
    assert_eq!(sources[0].get_hit_indices().len(), 1);
    assert_eq!(sources[1].get_hit_indices().len(), 1);
    assert_eq!(sources[0].get_hit_indices()[0], 0); // First hit index
    assert_eq!(sources[1].get_hit_indices()[0], 1); // Second hit index
}

#[test]
fn test_dto_add_edits() {
    let mut dto = DTO::new("test", "1.0", "author", "description");
    
    dto.add_edit(0, "First edit".to_string());
    dto.add_edit(10, "Second edit".to_string());
    
    assert_eq!(dto.get_edit_count(), 2);
    
    let edits = dto.get_edits();
    assert_eq!(edits[0].get_position(), 0);
    assert_eq!(edits[0].get_new_value(), "First edit");
    assert_eq!(edits[1].get_position(), 10);
    assert_eq!(edits[1].get_new_value(), "Second edit");
}

#[test]
fn test_transformation_phase_append_operation() {
    let mut dto = DTO::new("test", "1.0", "author", "description");
    dto.add_source("test_source", "Hello world");
    dto.add_hit("world".to_string(), 6, 0);
    
    // Simulate append transformation: "world" -> "world!"
    let original_value = dto.get_hits()[0].get_value();
    let new_value = format!("{}!", original_value);
    dto.add_edit(dto.get_hits()[0].get_position(), new_value);
    
    assert_eq!(dto.get_edit_count(), 1);
    assert_eq!(dto.get_edits()[0].get_new_value(), "world!");
}

#[test]
fn test_transformation_phase_replace_operation() {
    let mut dto = DTO::new("test", "1.0", "author", "description");
    dto.add_source("test_source", "Hello world");
    dto.add_hit("world".to_string(), 6, 0);
    
    // Simulate replace transformation: "world" -> "universe"
    dto.add_edit(dto.get_hits()[0].get_position(), "universe".to_string());
    
    assert_eq!(dto.get_edit_count(), 1);
    assert_eq!(dto.get_edits()[0].get_new_value(), "universe");
}

#[test]
fn test_multiple_sources_and_transformations() {
    let mut dto = DTO::new("test", "1.0", "author", "description");
    
    // Add multiple sources
    dto.add_source("file1.txt", "First file content with pattern");
    dto.add_source("file2.txt", "Second file also has pattern");
    
    // Add hits from both sources
    dto.add_hit("pattern".to_string(), 24, 0); // From first source
    dto.add_hit("pattern".to_string(), 21, 1); // From second source
    
    // Add edits for both hits
    dto.add_edit(24, "PATTERN".to_string()); // Transform first hit
    dto.add_edit(21, "PATTERN".to_string()); // Transform second hit
    
    assert_eq!(dto.get_source_count(), 2);
    assert_eq!(dto.get_hit_count(), 2);
    assert_eq!(dto.get_edit_count(), 2);
    
    // Verify source-hit associations
    let sources = dto.get_sources();
    assert_eq!(sources[0].get_hit_indices()[0], 0);
    assert_eq!(sources[1].get_hit_indices()[0], 1);
}
