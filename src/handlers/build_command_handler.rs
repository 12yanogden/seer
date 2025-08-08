use crate::arg_buliders::{
    AllArgBuilder, AppendArgBuilder, CountArgBuilder, CountBySourceArgBuilder, DirArgBuilder,
    EditInPlaceArgBuilder, EveryNthArgBuilder, ExcludeMatchesArgBuilder, FileArgBuilder,
    FilesArgBuilder, FindBetweenArgBuilder, FindRegexArgBuilder, FindStringArgBuilder,
    MaxDepthArgBuilder, NthArgBuilder, PrependArgBuilder, ReplaceWithArgBuilder, TextArgBuilder,
};
use crate::arg_group_builders::{
    EditEvaluateArgGroupBuilder, EditInPlaceArgGroupBuilder, EvaluateArgGroupBuilder,
    ExcludeMatchesArgGroupBuilder, FrequencyArgGroupBuilder, MaxDepthArgGroupBuilder,
    SearchArgGroupBuilder,
};
use crate::dto::dto::DTO;
use crate::handlers::handler::Handler;
use clap::Command;

pub trait CommandBuilder {
    fn build(cmd: &mut Command);
}

pub struct BuildCommandHandler;

impl BuildCommandHandler {
    pub fn new() -> Self {
        Self
    }
}

impl Handler for BuildCommandHandler {
    fn handle(&mut self, dto: &mut DTO) {
        let mut cmd = dto_to_command(&dto);

        // Add arguments
        AllArgBuilder::build(&mut cmd);
        AppendArgBuilder::build(&mut cmd);
        CountArgBuilder::build(&mut cmd);
        CountBySourceArgBuilder::build(&mut cmd);
        DirArgBuilder::build(&mut cmd);
        EditInPlaceArgBuilder::build(&mut cmd);
        EveryNthArgBuilder::build(&mut cmd);
        ExcludeMatchesArgBuilder::build(&mut cmd);
        FileArgBuilder::build(&mut cmd);
        FilesArgBuilder::build(&mut cmd);
        FindBetweenArgBuilder::build(&mut cmd);
        FindRegexArgBuilder::build(&mut cmd);
        FindStringArgBuilder::build(&mut cmd);
        MaxDepthArgBuilder::build(&mut cmd);
        NthArgBuilder::build(&mut cmd);
        PrependArgBuilder::build(&mut cmd);
        ReplaceWithArgBuilder::build(&mut cmd);
        TextArgBuilder::build(&mut cmd);

        // Add argument groups
        EditEvaluateArgGroupBuilder::build(&mut cmd);
        EditInPlaceArgGroupBuilder::build(&mut cmd);
        EvaluateArgGroupBuilder::build(&mut cmd);
        ExcludeMatchesArgGroupBuilder::build(&mut cmd);
        FrequencyArgGroupBuilder::build(&mut cmd);
        MaxDepthArgGroupBuilder::build(&mut cmd);
        SearchArgGroupBuilder::build(&mut cmd);

        dto.set_inputs(cmd.get_matches());
    }
}

/// Build a `clap::Command` from a DTO, including all arguments and argument groups.
///
/// # Example
/// ```
/// use seer::dto::dto::DTO;
/// use seer::handlers::build_command_handler::dto_to_command;
///
/// let dto = DTO::new("test_command", "1.0", "author_name", "description");
/// let cmd = dto_to_command(&dto);
/// assert_eq!(cmd.get_name(), "test_command");
/// ```
pub fn dto_to_command(dto: &DTO) -> clap::Command {
    let cmd_data = dto.clone_command_data();

    let name: &'static str = Box::leak(cmd_data.clone_name().into_boxed_str());
    let version: &'static str = Box::leak(cmd_data.clone_version().into_boxed_str());
    let author: &'static str = Box::leak(cmd_data.clone_author().into_boxed_str());
    let about: &'static str = Box::leak(cmd_data.clone_about().into_boxed_str());

    let cmd = clap::Command::new(name)
        .version(version)
        .author(author)
        .about(about);

    cmd
}

/// Applies a list of argument builder functions to a `clap::Command` created from the given DTO.
///
/// # Example
/// ```
/// use seer::dto::dto::DTO;
/// use seer::handlers::build_command_handler::{apply_arg_builders, dto_to_command};
/// use seer::arg_buliders::text_arg_builder::TextArgBuilder;
/// use clap::Command;
///
/// let dto = DTO::new("test_command", "1.0", "author_name", "description");
/// let builders: Vec<fn(&mut Command)> = vec![TextArgBuilder::build];
/// let mut cmd = apply_arg_builders(&dto, builders);
/// // Now `cmd` has the --text argument
/// let matches = cmd.try_get_matches_from(vec!["test_command", "--text", "hello"]);
/// assert!(matches.is_ok());
/// ```
pub fn apply_arg_builders(dto: &mut DTO, arg_builders: Vec<fn(&mut Command)>) -> clap::Command {
    let mut cmd = dto_to_command(dto);
    for builder in arg_builders {
        builder(&mut cmd);
    }
    cmd
}
