use super::command_data::CommandData;
use super::edit::Edit;
use super::evaluation::Evaluation;
use super::hit::Hit;
use super::source::Source;
use clap::ArgMatches;

pub struct DTO<'a> {
    command_data: CommandData,
    inputs: Option<clap::ArgMatches>,
    sources: Vec<Source<'a>>,
    hits: Vec<Hit>,
    edits: Vec<Edit>,
    evalutation: Option<&'a Evaluation<'a>>,
}

impl<'a> DTO<'a> {
    // Constructor for DTO
    pub fn new<'b>(
        command_name: &'b str,
        version: &'b str,
        author: &'b str,
        about: &'b str,
    ) -> Self {
        Self {
            command_data: CommandData::new(command_name, version, author, about),
            inputs: None,
            sources: Vec::new(),
            hits: Vec::new(),
            edits: Vec::new(),
            evalutation: None,
        }
    }

    // Adders

    // Add a source to the DTO.
    pub fn add_source(&mut self, name: &'a str, text: &'a str) {
        let source = Source::new(name, text);
        self.sources.push(source);
    }

    // Add a hit to the DTO.
    pub fn add_hit(&mut self, value: &'a str, position: usize, source: &'a mut Source<'a>) {
        let hit = Hit::new(value, position);
        self.hits.push(hit);
        let hit_index = self.hits.len() - 1;
        source.add_hit_index(hit_index);
    }

    // Cloners

    pub fn clone_command_data(&self) -> CommandData {
        self.command_data.clone()
    }

    // Getters

    pub fn get_command_data(&self) -> &CommandData {
        &self.command_data
    }

    pub fn get_inputs(&self) -> &Option<ArgMatches> {
        &self.inputs
    }

    pub fn get_sources(&self) -> &Vec<Source<'a>> {
        &self.sources
    }

    pub fn get_hits(&'a self) -> &'a Vec<Hit<'a>> {
        &self.hits
    }

    pub fn get_edits(&'a self) -> &'a Vec<Edit<'a>> {
        &self.edits
    }

    pub fn get_evaluation(&'a self) -> Option<&'a Evaluation<'a>> {
        self.evalutation
    }

    // Metadata getters

    pub fn get_source_count(&self) -> usize {
        self.sources.len()
    }

    pub fn get_hit_count(&self) -> usize {
        self.hits.len()
    }

    pub fn get_edit_count(&self) -> usize {
        self.edits.len()
    }

    // Precision getters

    // Get the value of an input argument by name, generic over type.
    pub fn get_input<T: 'static + Send + Sync + Clone>(&self, name: &str) -> Option<&T> {
        self.inputs
            .as_ref()
            .and_then(|inputs| inputs.get_one::<T>(name))
    }

    // Setters

    // Set the inputs by value
    pub fn set_inputs(&mut self, inputs: clap::ArgMatches) {
        self.inputs = Some(inputs);
    }
}
