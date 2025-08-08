use crate::dto::dto::DTO;

/// Trait for output strategies that define how processed data should be output
pub trait OutputStrategy {
    /// Apply the output strategy to the DTO
    /// 
    /// # Arguments
    /// 
    /// * `dto` - The data transfer object containing all processed data
    /// 
    /// # Returns
    /// 
    /// Result indicating success or error
    fn output(&self, dto: &DTO) -> Result<(), Box<dyn std::error::Error>>;
}
