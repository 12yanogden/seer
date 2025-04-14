#[derive(Debug)]
pub struct Hit {
    value: String,
    position: usize,
}

impl Hit {
    // Constructor for Hit
    pub fn new(value: String, position: usize) -> Self {
        Self { value, position }
    }

    // Returns the length of the string value.
    pub fn get_length(&self) -> usize {
        self.value.len()
    }

    // Returns the position integer.
    pub fn get_position(&self) -> usize {
        self.position
    }

    // Returns the end position (position + length).
    pub fn get_end_position(&self) -> usize {
        self.position + self.get_length() - 1
    }

    // Returns the value string.
    pub fn get_value(&self) -> &str {
        &self.value
    }

    // Sets the value string.
    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }

    // Sets the position integer.
    pub fn set_position(&mut self, position: usize) {
        self.position = position;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hit_length() {
        let h = Hit::new(String::from("test"), 5);
        assert_eq!(h.get_length(), 4);
    }

    #[test]
    fn test_hit_position() {
        let h = Hit::new(String::from("test"), 5);
        assert_eq!(h.get_position(), 5);
    }

    #[test]
    fn test_hit_end_position() {
        let h = Hit::new(String::from("test"), 5);
        assert_eq!(h.get_end_position(), 8);
    }

    #[test]
    fn test_hit_value() {
        let h = Hit::new(String::from("test"), 5);
        assert_eq!(h.get_value(), "test");
    }

    #[test]
    fn test_set_value() {
        let mut h = Hit::new(String::from("test"), 5);
        h.set_value(String::from("new_value"));
        assert_eq!(h.get_value(), "new_value");
    }

    #[test]
    fn test_set_position() {
        let mut h = Hit::new(String::from("test"), 5);
        h.set_position(10);
        assert_eq!(h.get_position(), 10);
    }
}
