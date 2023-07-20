use std::collections::BTreeMap;

use serde::Deserialize;

use super::location::Location;
use super::storage_option::StorageOption;

/// `GlobalResources` represents a collection of available locations and storages.
#[derive(Deserialize, Debug, PartialEq)]
pub struct GlobalResources {
    pub locations: BTreeMap<String, Location>,
    pub storage: BTreeMap<String, StorageOption>,
}
