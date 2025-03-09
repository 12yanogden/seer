pub struct Hit {
    value: String,
    position: usize,
}

impl Hit {
    /// Returns the length of the string value.
    pub fn length(&self) -> usize {
        self.value.len()
    }

    /// Returns the position integer.
    pub fn position(&self) -> usize {
        self.position
    }

    /// Returns the end position (position + length).
    pub fn end_position(&self) -> usize {
        self.position + self.length()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hit_length() {
        let h = Hit {
            value: String::from("test"),
            position: 5,
        };
        assert_eq!(h.length(), 4);
    }

    #[test]
    fn test_hit_position() {
        let h = Hit {
            value: String::from("test"),
            position: 5,
        };
        assert_eq!(h.position(), 5);
    }

    #[test]
    fn test_hit_end_position() {
        let h = Hit {
            value: String::from("test"),
            position: 5,
        };
        assert_eq!(h.end_position(), 9);
    }
}
