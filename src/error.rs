use std::fmt;
use std::fmt::Display;

/// An `Error` type for reporting errors related to TensorDock API interactions.
#[derive(Debug)]
pub struct TensorDockAPIError {
    pub message: String,
}

impl Display for TensorDockAPIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Encountered TensorDock API Error: {}", self.message)
    }
}
