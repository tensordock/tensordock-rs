use std::collections::HashMap;

use reqwest::Client;
use util::defaults::default_client;

pub mod config;
pub mod error;
mod util;

/// Client for the TensorDock Marketplace API, found here:
/// https://documenter.getpostman.com/view/20973002/2s8YzMYRDc
pub struct TensorDock {
    pub config: config::Config,
    client: Client,
}

impl TensorDock {
    /// Create a new `TensorDock` client with the given authorization
    /// key and authorization token.
    pub fn new(config: config::Config) -> Self {
        Self {
            config,
            client: default_client(),
        }
    }

    /// Test the authorization key and token to determine that the
    /// authorization is registered and valid. The endpoint returns
    /// `true` if the authorization is registered and valid.
    /// Endpoint: https://marketplace.tensordock.com/api/v0/auth/test
    pub async fn test(self) -> Result<bool, Box<dyn std::error::Error>> {
        let url = "https://marketplace.tensordock.com/api/v0/auth/test".parse::<reqwest::Url>()?;

        let api_key = self.config.key.as_str();
        let api_token = self.config.token.as_str();

        let params: HashMap<&str, &str> =
            HashMap::from([("api_key", api_key), ("api_token", api_token)]);

        let response = self.client.get(url).form(&params).send().await?;

        let json = response.json::<serde_json::Value>().await?;

        match json.as_bool() {
            Some(true) => Ok(true),
            _ => Ok(false),
        }
    }
}
