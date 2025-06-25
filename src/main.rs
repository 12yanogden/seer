use seer::arg_buliders::build_command;
use seer::handlers::build_command_handler::BuildCommandHandler;
use seer::handlers::build_dto_handler::CommandData;

/// The main function.
///
/// # Examples
///
/// ```
/// cargo run -- --find_string "foo" --text "foobar"
/// ```
fn main() {
    // TODO: Pass command data to dto and then pass dto to BuildCommandHandler
    // TODO: BuildCommandHandler can read any piped data from stdin and add it to the dto
    let dto = DTO::new(
        "seer",
        "1.0",
        "Ryan Ogden",
        "Search, Edit, Evaluate, and Replace text.",
    );

    BuildCommandHandler::new().build_command(&mut dto);
    // ValidateHandler::new().validate(&mut dto);
    GetSourcesHandler::new().get_sources(&mut dto);
}
