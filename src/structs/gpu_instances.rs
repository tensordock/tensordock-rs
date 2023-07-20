use serde::Deserialize;

/// A container for the units of expected cost for RAM and vCPUs for GPU
/// instances.
#[derive(Deserialize, PartialEq, Debug)]
pub struct GpuInstances {
    pub ram: GpuInstancesDescription,
    pub vcpu: GpuInstancesDescription,
}

/// A description of the costs for GPU instances.
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GpuInstancesDescription {
    /// The expected hourly cost of a GPU instance.
    pub cost_hr: f64,
    /// The unit of measurement against which the expected hourly cost is
    /// multiplied.
    pub unit: String,
}
