use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::ocsf::validation;

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ValidateEventRequest {
    pub event_json: String,
}

/// Validate an OCSF event JSON against the schema
pub async fn validate_ocsf_event(request: ValidateEventRequest) -> Result<String> {
    tracing::info!("validate_ocsf_event called");

    let report = validation::validate_event(&request.event_json)
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;

    serde_json::to_string_pretty(&report).map_err(|e| anyhow::anyhow!(e.to_string()))
}
