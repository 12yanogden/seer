pub struct Source<'a> {
    name: &'a str,
    text: &'a str,
    hit_indices: Vec<usize>,
}

impl<'a> Source<'a> {
    // Constructor for Source
    pub fn new(name: &'a str, text: &'a str) -> Self {
        Self {
            name,
            text,
            hit_indices: Vec::new(),
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
    pub fn set_name(&mut self, name: &'a str) {
        self.name = name;
    }

    // Sets the text string.
    pub fn set_text(&mut self, text: &'a str) {
        self.text = text;
    }

    // Get the hits for the source.
    pub fn get_hit_indices(&self) -> &Vec<usize> {
        &self.hit_indices
    }

    // Adds a hit to the hits vector.
    pub fn add_hit_index(&mut self, hit_index: usize) {
        self.hit_indices.push(hit_index);
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

    #[test]
    fn test_add_hit_index() {
        let mut source = Source::new("name", "text");
        let index = 3;
        source.add_hit_index(index);
        assert_eq!(source.hit_indices.len(), 1);
        assert_eq!(source.hit_indices[0], index);
    }
}
