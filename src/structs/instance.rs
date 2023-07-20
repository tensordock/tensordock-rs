use std::collections::BTreeMap;

use serde::Deserialize;

use super::specs::{CpuSpecs, GpuSpecs};

/// A `CpuInstance` represents an available CPU instance, containing
/// information about costs for certain lengths, available locations,
/// restrictions on usage, and specs of the instance.
#[derive(Deserialize, PartialEq, Debug)]
pub struct CpuInstance {
    pub name: String,
    pub cost: BTreeMap<String, f64>,
    pub locations: Vec<String>,
    pub restrictions: BTreeMap<String, u32>,
    pub specs: CpuSpecs,
}

/// A `GpuInstance` represents an available GPU instance, containing
/// information about costs for certain lengths, available locations,
/// restrictions on usage, and specs of the instance.
#[derive(Deserialize, PartialEq, Debug)]
pub struct GpuInstance {
    pub name: String,
    pub cost: BTreeMap<String, f64>,
    pub locations: Vec<String>,
    pub restrictions: BTreeMap<String, u32>,
    pub specs: GpuSpecs,
}
