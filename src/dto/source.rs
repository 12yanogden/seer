#[derive(Debug)]
pub struct Source {
    name: String,
    text: String,
}

impl Source {
    // Constructor for Source
    pub fn new(name: &str, text: &str) -> Self {
        Self {
            name: name.to_string(),
            text: text.to_string(),
        }
    }

    // Returns the name of the source.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    // Returns the text of the source.
    pub fn get_text(&self) -> &str {
        &self.text
    }

    // Sets the value string.
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    // Sets the text string.
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let source = Source::new("test_name", "test_text");
        assert_eq!(source.get_name(), "test_name");
        assert_eq!(source.get_text(), "test_text");
    }

    #[test]
    fn test_set_name() {
        let mut source = Source::new("initial_name", "text");
        source.set_name("new_name");
        assert_eq!(source.get_name(), "new_name");
    }

    #[test]
    fn test_set_text() {
        let mut source = Source::new("name", "initial_text");
        source.set_text("new_text");
        assert_eq!(source.get_text(), "new_text");
    }
}
