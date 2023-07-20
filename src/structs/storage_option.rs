use serde::Deserialize;

/// A `StorageOption` represents an option for storage, like an SSD or HDD.
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StorageOption {
    /// The hourly expected cost of using the given storage option.
    pub cost_hr: f64,
    /// A description of the type of storage option.
    pub description: String,
    /// A collection of locations available for the given storage option.
    pub locations: Vec<String>,
    pub unit: String,
}
