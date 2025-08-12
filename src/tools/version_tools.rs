use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::ocsf::OcsfSchema;

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ListVersionsRequest {}

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct GetNewestVersionRequest {}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionsResponse {
    pub versions: Vec<String>,
    pub count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewestVersionResponse {
    pub version: String,
    pub is_stable: bool,
}

/// List all available OCSF schema versions
pub async fn list_ocsf_versions(_request: ListVersionsRequest) -> Result<String> {
    tracing::info!("list_ocsf_versions called");

    let versions = OcsfSchema::list_versions()?;
    let response = VersionsResponse {
        count: versions.len(),
        versions,
    };

    serde_json::to_string_pretty(&response).map_err(|e| anyhow::anyhow!(e))
}

/// Get the newest stable OCSF schema version
pub async fn get_newest_ocsf_version(_request: GetNewestVersionRequest) -> Result<String> {
    tracing::info!("get_newest_ocsf_version called");

    let version = OcsfSchema::get_newest_stable_version()?;
    let response = NewestVersionResponse {
        version,
        is_stable: true,
    };

    serde_json::to_string_pretty(&response).map_err(|e| anyhow::anyhow!(e))
}
