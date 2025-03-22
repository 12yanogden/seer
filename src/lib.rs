// Modules
mod command;
mod file_sys;
pub mod frequency_strategies;
pub mod hit;
mod parse;
pub mod search;
pub mod search_strategies;
mod validate;

// Methods and Implementations
pub use command::init;
pub use file_sys::{get_file_paths_from_dir, read_file};
pub use hit::Hit;
pub use parse::{get_searchable, read_pipe};
pub use validate::{
    validate_input, verify_required_option_for_dependent_flag, verify_searchable_or_pipe_is_given,
};
