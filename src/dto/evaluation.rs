pub struct Evaluation<'a> {
    value: &'a str,
}

impl<'a> Evaluation<'a> {
    pub fn new(value: &'a str) -> Self {
        Evaluation { value }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
