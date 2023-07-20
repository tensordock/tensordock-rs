use serde::Deserialize;

/// A `CpuSpecs` represents the specifications of a CPU instance.
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CpuSpecs {
    pub avx1: bool,
    pub avx2: bool,
    pub avx512: bool,
    pub gbps: u32,
    pub ram: u32,
}

/// A `GpuSpecs` represents the specifications of a GPU instance.
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GpuSpecs {
    pub cuda_cores: u32,
    pub gbps: u32,
    pub nvlink: bool,
    pub single_precision_performance: f64,
    pub vram: u32,
}
