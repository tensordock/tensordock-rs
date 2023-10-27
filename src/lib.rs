pub mod config;
pub mod error;

/// Client for the TensorDock Marketplace API, found here:
/// https://documenter.getpostman.com/view/20973002/2s8YzMYRDc
pub struct TensorDock {
    pub config: config::Config,
}

impl TensorDock {
    /// Create a new `TensorDock` client with the given authorization
    /// key and authorization token.
    pub fn new(config: config::Config) -> Self {
        Self {
            config,
        }
    }
}
