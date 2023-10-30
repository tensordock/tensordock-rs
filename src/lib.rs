//! # tensordock-rs
//!
//! `tensordock-rs` is an API wrapper for the TensorDock Marketplace API to
//! facilitate deployment and management of Cloud GPU servers. TensorDock makes
//! cheap cloud GPU servers readily available to customers.
//!
//! ## Websites
//!
//! - [TensorDock] -- TensorDock's website
//! - [API Endpoint Documentation] -- documentation about available endpoints
//! and their expected request and response objects.
//! - [API Wrapper Documentation] -- documentation about this API wrapper.
//! - [API Keys] -- where to obtain authorization keys and tokens for the
//! TensorDock Marketplace.
//!
//! [TensorDock]: https://tensordock.com
//! [API Endpoint Documentation]: https://documenter.getpostman.com/view/20973002/2s8YzMYRDc
//! [API Wrapper Documentation]: https://docs.rs/tensordock-rs/latest/tensordock_rs/
//! [API Keys]: https://marketplace.tensordock.com/api
//!
//! ## Supported endpoints
//!
//! The currently supported endpoints by this API Wrapper are listed below.
//! Other endpoints are in-progress.
//!
//! - /api/v0/auth/test

use std::collections::HashMap;

use reqwest::Client;
use util::defaults::default_client;

pub mod config;
pub mod error;
mod util;

/// Client for the TensorDock Marketplace API, found here:
/// <https://documenter.getpostman.com/view/20973002/2s8YzMYRDc>
///
/// This struct exposes methods for interacting directly with the TensorDock
/// Marketplace API, requiring a configuration to be constructed containing a
/// valid authorization key and token. Authorization keys and tokens can be
/// generated from <https://marketplace.tensordock.com/api>.
pub struct TensorDock {
    /// The configuration for the TensorDock client, containing the
    /// authorization key and token.
    pub config: config::Config,
    client: Client,
}

impl TensorDock {
    /// Create a new `TensorDock` client with the given configuration that
    /// contains a valid authorization key and token.
    pub fn new(config: config::Config) -> Self {
        Self {
            config,
            client: default_client(),
        }
    }

    /// Test the authorization key and token to determine that the authorization
    /// is registered and valid. A POST request is made to the endpoint with the
    /// authorization key and token. The endpoint then returns `true` if the
    /// authorizations are registered and valid. Endpoint:
    /// <https://marketplace.tensordock.com/api/v0/auth/test>
    pub async fn test(self) -> Result<bool, Box<dyn std::error::Error>> {
        let url = "https://marketplace.tensordock.com/api/v0/auth/test".parse::<reqwest::Url>()?;

        let api_key = self.config.key.as_str();
        let api_token = self.config.token.as_str();

        let params: HashMap<&str, &str> =
            HashMap::from([("api_key", api_key), ("api_token", api_token)]);

        let response = self.client.get(url).form(&params).send().await?;

        let json = response.json::<serde_json::Value>().await?;

        let response_val = json.as_object()
            .and_then(|obj| obj.get("success"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        Ok(response_val)
    }
}
