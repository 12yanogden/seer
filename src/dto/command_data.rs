#[derive(Clone)]
pub struct CommandData {
    name: String,
    version: String,
    author: String,
    about: String,
}

impl CommandData {
    pub fn new(name: &str, version: &str, author: &str, about: &str) -> Self {
        CommandData {
            name: String::from(name),
            version: String::from(version),
            author: String::from(author),
            about: String::from(about),
        }
    }

    // Getters

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_version(&self) -> &str {
        &self.version
    }

    pub fn get_author(&self) -> &str {
        &self.author
    }

    pub fn get_about(&self) -> &str {
        &self.about
    }

    // Clones

    pub fn clone_name(&self) -> String {
        self.name.clone()
    }

    pub fn clone_version(&self) -> String {
        self.version.clone()
    }

    pub fn clone_author(&self) -> String {
        self.author.clone()
    }

    pub fn clone_about(&self) -> String {
        self.about.clone()
    }
}
