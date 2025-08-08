use super::command_data::CommandData;
use super::edit::Edit;
use super::hit::Hit;
use super::source::Source;
use clap::ArgMatches;

#[derive(Clone)]
pub struct DTO {
    command_data: CommandData,
    inputs: Option<clap::ArgMatches>,
    sources: Vec<Source>,
    hits: Vec<Hit>,
    edits: Vec<Edit>,
}

impl DTO {
    // Constructor for DTO
    pub fn new(
        command_name: &str,
        version: &str,
        author: &str,
        about: &str,
    ) -> Self {
        Self {
            command_data: CommandData::new(command_name, version, author, about),
            inputs: None,
            sources: Vec::new(),
            hits: Vec::new(),
            edits: Vec::new(),
        }
    }

    // Adders

    // Add a source to the DTO.
    pub fn add_source(&mut self, name: &str, text: &str) {
        let source = Source::new(name, text);
        self.sources.push(source);
    }

    // Add a hit to the DTO.
    pub fn add_hit(&mut self, value: String, position: usize, source_index: usize) {
        let hit = Hit::new(value, position);
        self.hits.push(hit);
        let hit_index = self.hits.len() - 1;
        if source_index < self.sources.len() {
            self.sources[source_index].add_hit_index(hit_index);
        }
    }

    // Add an edit to the DTO.
    pub fn add_edit(&mut self, position: usize, new_value: String) {
        let edit = Edit::new(position, new_value);
        self.edits.push(edit);
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

    pub fn get_sources(&self) -> &Vec<Source> {
        &self.sources
    }

    pub fn get_hits(&self) -> &Vec<Hit> {
        &self.hits
    }

    pub fn get_edits(&self) -> &Vec<Edit> {
        &self.edits
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
