#[derive(Debug)]
pub struct Hit {
    position: usize,
    length: usize,
}

impl Hit {
    // Constructor for Hit
    pub fn new(position: usize, length: usize) -> Self {
        Self { position, length }
    }

    // Returns the length.
    pub fn get_length(&self) -> usize {
        self.length
    }

    // Returns the position integer.
    pub fn get_position(&self) -> usize {
        self.position
    }

    // Returns the end position (position + length - 1).
    pub fn get_end_position(&self) -> usize {
        self.position + self.length - 1
    }

    // Sets the length.
    pub fn set_length(&mut self, length: usize) {
        self.length = length;
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
        let h = Hit::new(5, 4);
        assert_eq!(h.get_length(), 4);
    }

    #[test]
    fn test_hit_position() {
        let h = Hit::new(5, 4);
        assert_eq!(h.get_position(), 5);
    }

    #[test]
    fn test_hit_end_position() {
        let h = Hit::new(5, 4);
        assert_eq!(h.get_end_position(), 8);
    }

    #[test]
    fn test_set_length() {
        let mut h = Hit::new(5, 4);
        h.set_length(10);
        assert_eq!(h.get_length(), 10);
    }

    #[test]
    fn test_set_position() {
        let mut h = Hit::new(5, 4);
        h.set_position(10);
        assert_eq!(h.get_position(), 10);
    }
}
