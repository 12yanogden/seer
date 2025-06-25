#[derive(Debug)]
pub struct Edit {
    position: usize,
    new_value: String,
}

impl Edit {
    // Constructor for Edit
    pub fn new(position: usize, new_value: String) -> Self {
        Self {
            position,
            new_value,
        }
    }

    // Returns the position integer.
    pub fn get_position(&self) -> usize {
        self.position
    }

    // Returns a reference to the new_value string.
    pub fn get_new_value(&self) -> &str {
        &self.new_value
    }

    // Sets the position integer.
    pub fn set_position(&mut self, position: usize) {
        self.position = position;
    }

    // Sets the new_value string.
    pub fn set_new_value<S: Into<String>>(&mut self, new_value: S) {
        self.new_value = new_value.into();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edit_position() {
        let e = Edit::new(5, "foo".to_string());
        assert_eq!(e.get_position(), 5);
    }

    #[test]
    fn test_edit_new_value() {
        let e = Edit::new(5, "foo".to_string());
        assert_eq!(e.get_new_value(), "foo");
    }

    #[test]
    fn test_set_position() {
        let mut e = Edit::new(5, "foo".to_string());
        e.set_position(10);
        assert_eq!(e.get_position(), 10);
    }

    #[test]
    fn test_set_new_value() {
        let mut e = Edit::new(5, "foo".to_string());
        e.set_new_value("bar");
        assert_eq!(e.get_new_value(), "bar");
    }
}
