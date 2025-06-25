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

impl<'a> Handler<'a> for BuildCommandHandler {
    fn handle(&mut self, dto: &mut DTO<'a>) {
        let cmd_data = dto.clone_command_data();

        let name: &'static str = Box::leak(cmd_data.clone_name().into_boxed_str());
        let version: &'static str = Box::leak(cmd_data.clone_version().into_boxed_str());
        let author: &'static str = Box::leak(cmd_data.clone_author().into_boxed_str());
        let about: &'static str = Box::leak(cmd_data.clone_about().into_boxed_str());

        let mut cmd = Command::new(name)
            .version(version)
            .author(author)
            .about(about);

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
