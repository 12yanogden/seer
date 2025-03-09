mod command;
mod file_sys;
mod hit;
mod parse;
mod validate;
pub use command::init;
pub use file_sys::{get_file_paths_from_dir, read_file};
pub use hit::Hit;
pub use parse::{get_searchable, read_pipe};
pub use validate::{
    validate_input, verify_required_option_for_dependent_flag, verify_searchable_or_pipe_is_given,
};
