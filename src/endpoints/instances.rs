use reqwest;
use serde_json;

use crate::structs::instances_response::InstancesResponse;

#[derive(Debug)]

pub enum Error {
    ReqwestError(reqwest::Error),
    SerdeError(serde_json::Error),
}

/// Query the `/api/metadata/instances` endpoint and produce an `InstancesResponse` or an
/// `InstancesError`.
///
/// # Tensordock API Docs:
///
/// Here is where you can retrieve data about the restrictions placed on instances.
/// 
/// ## CPU Instances:
///
/// You can retrieve the cost, the locations where a specific type of CPU can be deployed, and the
/// specifications.
/// 
/// CPU instances come in multiples of 1 vCPU and 4 GB of RAM. We guarantee that you can utilize
/// your CPU 24x7 without any throttling. Each multiple costs the costHr, and you can combine
/// multiples up to the maxMultiples number shown.
/// 
/// ## GPU Instances:
/// 
/// You can get the cost, the name, some performance metrics, the restrictions, as well as the
/// locations that they are available in.
/// 
/// ## Notes
/// 
/// Units can be interpreted through common sense, unless otherwise noted. E.g. 4 for a RAM field
/// translates to 4GB of RAM, and 10 for the network port translates to a 10 Gbps (1.25 GB/s) port.
///
/// # Errors
/// Will return `Err` if
/// - `reqwest` produces an `Err` from querying the endpoint
/// - `serde_json` fails to parse the response's body
pub async fn get() -> Result<InstancesResponse, Error> {
    match get_raw_instances_body().await {
        Ok(s) => parse_raw_instances_body(&s),
        Err(e) => Err(Error::ReqwestError(e)),
    }
}

async fn get_raw_instances_body() -> Result<String, reqwest::Error> {
    let raw_body = reqwest::get("https://console.tensordock.com/api/metadata/instances")
        .await?
        .text()
        .await?;

    Ok(raw_body)
}

fn parse_raw_instances_body(body: &str) -> Result<InstancesResponse, Error> {
    let instances_result: Result<InstancesResponse, serde_json::Error> =
        serde_json::from_str(body);
    match instances_result {
        Ok(instances) => Ok(instances),
        Err(e) => Err(Error::SerdeError(e)),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::structs::{
        global_resources::GlobalResources,
        gpu_instances::{GpuInstances, GpuInstancesDescription},
        instance::{CpuInstance, GpuInstance},
        instances_response::InstancesResponse,
        location::Location,
        resources::Resources,
        specs::{CpuSpecs, GpuSpecs},
        storage_option::StorageOption,
    };

    use super::parse_raw_instances_body;

    #[test]
    fn test_basic_parse() {
        // Arrange
        let body = "
{
  \"cpu\": {
    \"AMD_EPYC_MILAN\": {
      \"cost\": {
        \"costCommit1Yr\": 282.51,
        \"costCommit2Yr\": 452.02,
        \"costHr\": 0.043
      },
      \"locations\": [
        \"na-us-las-1\",
        \"na-us-nyc-1\",
        \"na-us-chi-1\"
      ],
      \"name\": \"AMD EPYC Milan\",
      \"restrictions\": {
        \"maxMultiples\": 44,
        \"maxRAMPerInstance\": 176
      },
      \"specs\": {
        \"avx1\": true,
        \"avx2\": true,
        \"avx512\": false,
        \"gbps\": 10,
        \"ram\": 4
      }
    }
  },
  \"gpu\": {
    \"A100_40GB\": {
      \"cost\": {
        \"costCommit1Yr\": 14913.9,
        \"costCommit2Yr\": 23862.24,
        \"costHr\": 2.27
      },
      \"locations\": [
        \"na-us-chi-1\"
      ],
      \"name\": \"NVIDIA A100 40GB for PCIE\",
      \"restrictions\": {
        \"maxGPUsPerInstance\": 8,
        \"maxRAMPerInstance\": 492,
        \"maxRAMPervCPU\": 24,
        \"maxvCPUsPerGPU\": 18,
        \"maxvCPUsPerInstance\": 94,
        \"minRamPervCPU\": 1,
        \"minvCPUsPerGPU\": 1
      },
      \"specs\": {
        \"cudaCores\": 6912,
        \"gbps\": 10,
        \"nvlink\": false,
        \"singlePrecisionPerformance\": 19.5,
        \"vram\": 40
      }
    }
  },
  \"resources\": {
    \"global\": {
      \"locations\": {
        \"na-us-bos-1\": {
          \"connectivity\": {},
          \"deployable\": false,
          \"name\": \"Boston DC 1\",
          \"reservable\": true,
          \"timezone\": \"UTC-5\"
        }
      },
      \"storage\": {
        \"io1\": {
          \"costHr\": 0.0001,
          \"description\": \"Perfomance-optimized NVMe SSD\",
          \"locations\": [
            \"na-us-bos-1\",
            \"na-us-bos-2\",
            \"na-us-chi-1\",
            \"na-us-las-1\",
            \"na-us-nyc-1\",
            \"oc-sg-sin-1\"
          ],
          \"unit\": \"per allocated GB of storage\"
        }
      }
    },
    \"gpu_instances\": {
      \"ram\": {
        \"costHr\": 0.005,
        \"unit\": \"per allocated GB of RAM\"
      },
      \"vcpu\": {
        \"costHr\": 0.01,
        \"unit\": \"per allocated vCPU\"
      }
    }
  },
  \"success\": true
}
";
        let expected = InstancesResponse {
            cpu: BTreeMap::from([(
                String::from("AMD_EPYC_MILAN"),
                CpuInstance {
                    name: String::from("AMD EPYC Milan"),
                    cost: BTreeMap::from([
                        (String::from("costCommit1Yr"), 282.51),
                        (String::from("costCommit2Yr"), 452.02),
                        (String::from("costHr"), 0.043),
                    ]),
                    locations: vec![
                        String::from("na-us-las-1"),
                        String::from("na-us-nyc-1"),
                        String::from("na-us-chi-1"),
                    ],
                    restrictions: BTreeMap::from([
                        (String::from("maxMultiples"), 44),
                        (String::from("maxRAMPerInstance"), 176),
                    ]),
                    specs: CpuSpecs {
                        avx1: true,
                        avx2: true,
                        avx512: false,
                        gbps: 10,
                        ram: 4,
                    },
                },
            )]),
            gpu: BTreeMap::from([(
                String::from("A100_40GB"),
                GpuInstance {
                    name: String::from("NVIDIA A100 40GB for PCIE"),
                    cost: BTreeMap::from([
                        (String::from("costCommit1Yr"), 14913.9),
                        (String::from("costCommit2Yr"), 23862.24),
                        (String::from("costHr"), 2.27),
                    ]),
                    locations: vec![String::from("na-us-chi-1")],
                    restrictions: BTreeMap::from([
                        (String::from("maxGPUsPerInstance"), 8),
                        (String::from("maxRAMPerInstance"), 492),
                        (String::from("maxRAMPervCPU"), 24),
                        (String::from("maxvCPUsPerGPU"), 18),
                        (String::from("maxvCPUsPerInstance"), 94),
                        (String::from("minRamPervCPU"), 1),
                        (String::from("minvCPUsPerGPU"), 1),
                    ]),
                    specs: GpuSpecs {
                        cuda_cores: 6912,
                        gbps: 10,
                        nvlink: false,
                        single_precision_performance: 19.5,
                        vram: 40,
                    },
                },
            )]),
            resources: Resources {
                global: GlobalResources {
                    locations: BTreeMap::from([(
                        String::from("na-us-bos-1"),
                        Location {
                            name: String::from("Boston DC 1"),
                            connectivity: BTreeMap::from([]),
                            deployable: false,
                            reservable: true,
                            timezone: String::from("UTC-5"),
                        },
                    )]),
                    storage: BTreeMap::from([(
                        String::from("io1"),
                        StorageOption {
                            cost_hr: 0.0001,
                            description: String::from("Perfomance-optimized NVMe SSD"),
                            locations: vec![
                                String::from("na-us-bos-1"),
                                String::from("na-us-bos-2"),
                                String::from("na-us-chi-1"),
                                String::from("na-us-las-1"),
                                String::from("na-us-nyc-1"),
                                String::from("oc-sg-sin-1"),
                            ],
                            unit: String::from("per allocated GB of storage"),
                        },
                    )]),
                },
                gpu_instances: GpuInstances {
                    ram: GpuInstancesDescription {
                        cost_hr: 0.005,
                        unit: String::from("per allocated GB of RAM"),
                    },
                    vcpu: GpuInstancesDescription {
                        cost_hr: 0.01,
                        unit: String::from("per allocated vCPU"),
                    },
                },
            },
            success: true,
        };

        // Act
        let result = parse_raw_instances_body(body);

        // Assert
        assert_eq!(result.unwrap(), expected);
    }
}
