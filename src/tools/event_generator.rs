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

    // Parse required_fields - can be JSON object or comma-separated field names
    let req_fields: HashMap<String, Value> = if request.required_fields.trim().starts_with('{') {
        // JSON object format
        serde_json::from_str(&request.required_fields)
            .map_err(|e| anyhow::anyhow!("Invalid JSON in required_fields: {}", e))?
    } else {
        // Comma-separated field names - generate default values
        let field_names: Vec<&str> = request.required_fields.split(',').map(|s| s.trim()).collect();
        let mut fields = HashMap::new();
        for field_name in field_names {
            if !field_name.is_empty() {
                let default_value = match field_name {
                    "activity_id" => Value::Number(serde_json::Number::from(1)),
                    "category_uid" => Value::Number(serde_json::Number::from(ec.uid / 1000)),
                    "class_uid" => Value::Number(serde_json::Number::from(ec.uid)),
                    "severity_id" => Value::Number(serde_json::Number::from(1)),
                    "type_uid" => Value::Number(serde_json::Number::from(ec.uid * 100 + 1)),
                    "time" => Value::String(chrono::Utc::now().to_rfc3339()),
                    _ => Value::String(format!("default_{}", field_name)),
                };
                fields.insert(field_name.to_string(), default_value);
            }
        }
        fields
    };

    let opt_fields: HashMap<String, Value> = if let Some(opt) = request.optional_fields {
        if opt.trim().starts_with('{') {
            // JSON object format
            serde_json::from_str(&opt)
                .map_err(|e| anyhow::anyhow!("Invalid JSON in optional_fields: {}", e))?
        } else {
            // Comma-separated field names - generate default values
            let field_names: Vec<&str> = opt.split(',').map(|s| s.trim()).collect();
            let mut fields = HashMap::new();
            for field_name in field_names {
                if !field_name.is_empty() {
                    let default_value = match field_name {
                        "message" => Value::String("Generated OCSF event".to_string()),
                        "user" => serde_json::json!({"name": "example_user", "uid": "1001"}),
                        _ => Value::String(format!("default_{}", field_name)),
                    };
                    fields.insert(field_name.to_string(), default_value);
                }
            }
            fields
        }
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
