use std::collections::BTreeMap;

use serde::Deserialize;

/// A Location represents an available resource location.
#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Location {
    pub name: String,
    /// Containing the endpoints to test connectivity with for performance, or
    /// an IP to test against.
    pub connectivity: BTreeMap<String, String>,
    /// Whether the location can be deployed as a server.
    pub deployable: bool,
    /// Whether the location is reservable as a server.
    pub reservable: bool,
    /// The timezone as a string, in a format like "UTC-6".
    pub timezone: String,
}
