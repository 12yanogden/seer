use clap::{Command, Arg, ArgGroup};

#[test]
fn test_transform_phase_edit_evaluation_incompatibility() {
    let cmd = Command::new("test_command")
        // Add Edit Transform operations
        .arg(Arg::new("append").long("append").value_name("TEXT").help("Append text to found matches"))
        .arg(Arg::new("prepend").long("prepend").value_name("TEXT").help("Prepend text to found matches"))
        .arg(Arg::new("replace-with").long("replace-with").value_name("TEXT").help("Replace found matches with text"))
        // Add Evaluation Transform operations
        .arg(Arg::new("count").long("count").action(clap::ArgAction::SetTrue).help("Count total matches"))
        .arg(Arg::new("count-by-source").long("count-by-source").action(clap::ArgAction::SetTrue).help("Count matches by source"))
        // Transform phase restrictions: Edit operations cannot be used with Evaluation operations
        .group(
            ArgGroup::new("transform_edit_group")
                .args(&["append", "prepend", "replace-with"]) // Edit Transform operations
                .conflicts_with("count")
                .conflicts_with("count-by-source")
                .multiple(false),
        )
        // Evaluation operations are mutually exclusive
        .group(
            ArgGroup::new("transform_evaluation_group")
                .args(&["count", "count-by-source"])
                .multiple(false),
        );
    
    // Test: Edit operations (append) cannot be used with Evaluation operations (count)
    let result = cmd.clone().try_get_matches_from(vec![
        "test_command", 
        "--append", "suffix", 
        "--count"
    ]);
    assert!(result.is_err(), "Transform phase: Edit and Evaluation operations should be incompatible");
    
    // Test: Edit operations (prepend) cannot be used with Evaluation operations (count-by-source)  
    let result = cmd.clone().try_get_matches_from(vec![
        "test_command",
        "--prepend", "prefix", 
        "--count-by-source"
    ]);
    assert!(result.is_err(), "Transform phase: Edit and Evaluation operations should be incompatible");
    
    // Test: Edit operations (replace-with) cannot be used with Evaluation operations (count)
    let result = cmd.clone().try_get_matches_from(vec![
        "test_command",
        "--replace-with", "replacement",
        "--count"
    ]);
    assert!(result.is_err(), "Transform phase: Edit and Evaluation operations should be incompatible");
}

#[test]
fn test_transform_phase_evaluation_mutual_exclusivity() {
    let cmd = Command::new("test_command")
        .arg(Arg::new("count").long("count").action(clap::ArgAction::SetTrue).help("Count total matches"))
        .arg(Arg::new("count-by-source").long("count-by-source").action(clap::ArgAction::SetTrue).help("Count matches by source"))
        // Transform phase: Evaluation operations are mutually exclusive
        .group(
            ArgGroup::new("transform_evaluation_group")
                .args(&["count", "count-by-source"])
                .multiple(false),
        );
    
    // Test: Evaluation operations are mutually exclusive
    let result = cmd.try_get_matches_from(vec![
        "test_command",
        "--count",
        "--count-by-source"
    ]);
    assert!(result.is_err(), "Transform phase: Evaluation operations should be mutually exclusive");
}

#[test]
fn test_transform_phase_valid_combinations() {
    let cmd = Command::new("test_command")
        .arg(Arg::new("append").long("append").value_name("TEXT").help("Append text to found matches"))
        .arg(Arg::new("count").long("count").action(clap::ArgAction::SetTrue).help("Count total matches"))
        // Transform phase restrictions
        .group(
            ArgGroup::new("transform_edit_group")
                .args(&["append"])
                .conflicts_with("count")
                .multiple(false),
        );
    
    // Test: Individual operations should work
    let result = cmd.clone().try_get_matches_from(vec![
        "test_command",
        "--append", "suffix"
    ]);
    assert!(result.is_ok(), "Individual Transform operations should work");
    
    let result = cmd.clone().try_get_matches_from(vec![
        "test_command",
        "--count"
    ]);
    assert!(result.is_ok(), "Individual Transform operations should work");
}
