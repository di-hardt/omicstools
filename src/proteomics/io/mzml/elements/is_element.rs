use anyhow::Result;

pub trait IsElement {
    /// Validates the mzML element
    ///
    fn validate(&self) -> Result<()>;
}
