use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::ocsf::{OcsfEvent, OcsfSchema};

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct GenerateEventRequest {
    #[schemars(description = "OCSF schema version (defaults to 1.7.0-dev)")]
    pub version: Option<String>,
    pub event_class: String,
    pub required_fields: String,
    pub optional_fields: Option<String>,
}

/// Generate a valid OCSF event JSON from parameters
pub async fn generate_ocsf_event(request: GenerateEventRequest) -> Result<String> {
    let version = request.version.as_deref().unwrap_or("1.7.0-dev");

    tracing::info!(
        "generate_ocsf_event called: version={}, event_class={}",
        version,
        request.event_class
    );

    let schema = OcsfSchema::load_version(version)
        .await
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;

    let ec = schema
        .get_event_class(&request.event_class)
        .ok_or_else(|| {
            anyhow::anyhow!(format!("Event class '{}' not found", request.event_class))
        })?;

    let req_fields: HashMap<String, Value> = serde_json::from_str(&request.required_fields)
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;

    let opt_fields: HashMap<String, Value> = if let Some(opt) = request.optional_fields {
        serde_json::from_str(&opt).map_err(|e| anyhow::anyhow!(e.to_string()))?
    } else {
        HashMap::new()
    };

    // Derive category_uid from class_uid (OCSF category is first digit of class UID)
    let category_uid = ec.uid / 1000;

    let mut event = OcsfEvent::new(&request.event_class, ec.uid, category_uid);

    for (key, value) in req_fields {
        event.set_field(key, value);
    }
    for (key, value) in opt_fields {
        event.set_field(key, value);
    }

    if !event.fields.contains_key("time") {
        event.set_field(
            "time".to_string(),
            Value::String(chrono::Utc::now().to_rfc3339()),
        );
    }

    event.to_json().map_err(|e| anyhow::anyhow!(e.to_string()))
}
