use seer::dto::dto::DTO;
use seer::handlers::build_command_handler::BuildCommandHandler;
use seer::handlers::get_sources_handler::GetSourcesHandler;
use seer::handlers::simple_search_handler::SimpleSearchHandler;
use seer::handlers::edit_handler::EditHandler;
use seer::handlers::evaluate_handler::EvaluateHandler;
use seer::handlers::print_handler::PrintHandler;
use seer::handlers::handler::Handler;

/// The main function.
///
/// # Examples
///
/// ```
/// cargo run -- --find-string "foo" --text "foobar"
/// ```
fn main() {
    let mut dto = DTO::new(
        "seer",
        "1.0",
        "Ryan Ogden",
        "Search, Edit, Evaluate, and Replace text.",
    );

    // Phase 1: Build command (parse arguments)
    BuildCommandHandler::new().handle(&mut dto);
    
    // Phase 2: Get sources (load input text from files, stdin, etc.)
    GetSourcesHandler::new().handle(&mut dto);
    
    // Phase 3: Find patterns
    SimpleSearchHandler::new().handle(&mut dto);
    
    // Phase 4: Transform (either edit or evaluate)
    if let Some(inputs) = dto.get_inputs() {
        // Check if this is an edit operation
        if inputs.contains_id("append") || inputs.contains_id("prepend") || inputs.contains_id("replace-with") {
            EditHandler::new().handle(&mut dto);
        }
        // Check if this is an evaluate operation
        else if inputs.contains_id("count") || inputs.contains_id("count-by-source") {
            EvaluateHandler::new().handle(&mut dto);
            return; // Evaluate operations print directly, no need for output phase
        }
    }
    
    // Phase 5: Output results
    PrintHandler::new().handle(&mut dto);
}
