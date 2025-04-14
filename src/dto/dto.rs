pub struct DTO {
    sources: Vec<Source>,
    hits: Vec<Hit>,
}

impl DTO {
    // Constructor for DTO
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
            hits: Vec::new(),
        }
    }

    // Returns the number of sources in the DTO.
    pub fn get_sources(&self) -> Vec<Source> {
        self.sources.len()
    }

    // Returns the number of hits in the DTO.
    pub fn get_hit_count(&self) -> Vec<Hit> {
        self.hits.len()
    }

    // Returns the sources of the DTO.
    pub fn get_sources(&self) -> Vec<Source> {
        self.sources
    }

    // Returns the hits of the DTO.
    pub fn get_hits(&self) -> Vec<Hit> {
        self.hits
    }

    // Add a source to the DTO.
    pub fn add_source(&mut self, source: Source) {
        self.sources.push(source);
    }

    // Add a hit to the DTO.
    pub fn add_hit(&mut self, hit: Hit) {
        self.hits.push(hit);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
