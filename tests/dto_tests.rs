use seer::dto::dto::DTO;
use seer::dto::hit::Hit;
use seer::dto::edit::Edit;
use seer::output_strategies::traits::output_strategy::OutputStrategy;
use seer::output_strategies::stdout_strategy::StdoutStrategy;

#[test]
fn test_dto_basic_functionality() {
    let mut dto = DTO::new("test", "1.0", "author", "description");
    
    // Test adding sources
    dto.add_source("source1", "Hello world");
    dto.add_source("source2", "Another source");
    
    assert_eq!(dto.get_source_count(), 2);
    assert_eq!(dto.get_sources()[0].get_name(), "source1");
    assert_eq!(dto.get_sources()[0].get_text(), "Hello world");
    assert_eq!(dto.get_sources()[1].get_name(), "source2");
    assert_eq!(dto.get_sources()[1].get_text(), "Another source");
}

#[test]
fn test_dto_hits_functionality() {
    let mut dto = DTO::new("test", "1.0", "author", "description");
    dto.add_source("source1", "Hello world");
    
    // Test adding hits
    dto.add_hit("world".to_string(), 6, 0);
    dto.add_hit("Hello".to_string(), 0, 0);
    
    assert_eq!(dto.get_hit_count(), 2);
    assert_eq!(dto.get_hits()[0].get_value(), "world");
    assert_eq!(dto.get_hits()[0].get_position(), 6);
    assert_eq!(dto.get_hits()[1].get_value(), "Hello");
    assert_eq!(dto.get_hits()[1].get_position(), 0);
    
    // Verify hit indices are tracked in source
    let source = &dto.get_sources()[0];
    assert_eq!(source.get_hit_indices().len(), 2);
    assert_eq!(source.get_hit_indices()[0], 0); // First hit index
    assert_eq!(source.get_hit_indices()[1], 1); // Second hit index
}

#[test]
fn test_dto_edits_functionality() {
    let mut dto = DTO::new("test", "1.0", "author", "description");
    
    // Test adding edits
    dto.add_edit(0, "Replacement1".to_string());
    dto.add_edit(10, "Replacement2".to_string());
    
    assert_eq!(dto.get_edit_count(), 2);
    assert_eq!(dto.get_edits()[0].get_position(), 0);
    assert_eq!(dto.get_edits()[0].get_new_value(), "Replacement1");
    assert_eq!(dto.get_edits()[1].get_position(), 10);
    assert_eq!(dto.get_edits()[1].get_new_value(), "Replacement2");
}

#[test]
fn test_stdout_output_strategy() {
    let mut dto = DTO::new("test", "1.0", "author", "description");
    dto.add_source("source1", "Hello world");
    dto.add_hit("world".to_string(), 6, 0);
    dto.add_edit(6, "universe".to_string());
    
    let strategy = StdoutStrategy::new();
    let result = strategy.output(&dto);
    
    // Should not error
    assert!(result.is_ok());
}

#[test] 
fn test_complete_transformation_pipeline() {
    let mut dto = DTO::new("test", "1.0", "author", "description");
    
    // Phase 1: Add sources (Source phase)
    dto.add_source("file1.txt", "The quick brown fox jumps over the lazy dog.");
    dto.add_source("file2.txt", "Another test string with fox in it.");
    
    // Phase 2: Find patterns (Find phase) 
    // Simulate finding "fox" in both files
    dto.add_hit("fox".to_string(), 16, 0); // Position 16 in file1
    dto.add_hit("fox".to_string(), 25, 1); // Position 25 in file2
    
    // Phase 3: Transform (Transform phase)
    // Replace "fox" with "wolf" 
    dto.add_edit(16, "wolf".to_string()); // Edit for first hit
    dto.add_edit(25, "wolf".to_string()); // Edit for second hit
    
    // Phase 4: Verify the DTO state
    assert_eq!(dto.get_source_count(), 2);
    assert_eq!(dto.get_hit_count(), 2);
    assert_eq!(dto.get_edit_count(), 2);
    
    // Verify source-hit associations
    assert_eq!(dto.get_sources()[0].get_hit_indices().len(), 1);
    assert_eq!(dto.get_sources()[1].get_hit_indices().len(), 1);
    assert_eq!(dto.get_sources()[0].get_hit_indices()[0], 0); // First hit belongs to first source
    assert_eq!(dto.get_sources()[1].get_hit_indices()[0], 1); // Second hit belongs to second source
    
    // Phase 5: Output (Output phase)
    let strategy = StdoutStrategy::new();
    let result = strategy.output(&dto);
    assert!(result.is_ok());
}
