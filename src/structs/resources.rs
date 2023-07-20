use serde::Deserialize;

use super::{global_resources::GlobalResources, gpu_instances::GpuInstances};

/// Resources represents the available resources to access.
#[derive(Deserialize, PartialEq, Debug)]
pub struct Resources {
    pub global: GlobalResources,
    pub gpu_instances: GpuInstances,
}
