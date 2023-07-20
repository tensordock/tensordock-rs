use std::collections::BTreeMap;

use serde::Deserialize;

use super::{
    instance::{CpuInstance, GpuInstance},
    resources::Resources,
};

/// An `InstancesResponse` represents a response of the `/api/metadata/instances`
/// endpoint, containing information about available CPUs, GPUs, and other
/// resources.
#[derive(Deserialize, PartialEq, Debug)]
pub struct InstancesResponse {
    pub cpu: BTreeMap<String, CpuInstance>,
    pub gpu: BTreeMap<String, GpuInstance>,
    pub resources: Resources,
    pub success: bool,
}
