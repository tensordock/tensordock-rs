use reqwest::{
    header::{HeaderMap, HeaderValue, USER_AGENT},
    Client, ClientBuilder,
};

/// Produce the default client for the TensorDock API.
pub fn default_client() -> Client {
    ClientBuilder::new()
        .default_headers(default_headers())
        .build()
        .expect("Failed to build default client.")
}

/// Produce the default headers for the TensorDock API.
pub fn default_headers() -> HeaderMap {
    HeaderMap::from_iter([(
        USER_AGENT,
        HeaderValue::from_static("tensordock/rust-client"),
    )])
}
