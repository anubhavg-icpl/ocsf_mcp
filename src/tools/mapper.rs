use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct MapCustomRequest {
    pub sample_log: String,
    pub suggested_class: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct MappingRecommendation {
    pub suggested_event_class: String,
    pub confidence: String,
    pub field_mappings: Vec<FieldMapping>,
    pub explanation: String,
}

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct FieldMapping {
    pub source_field: String,
    pub ocsf_field: String,
    pub transformation: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ListExamplesRequest {
    pub event_class: String,
    pub scenario: Option<String>,
}

/// Map custom log format to OCSF event class
pub async fn map_custom_to_ocsf(request: MapCustomRequest) -> Result<String> {
    tracing::info!("map_custom_to_ocsf called");

    let event_class = if let Some(suggested) = request.suggested_class {
        suggested
    } else {
        // Heuristic-based detection
        if request.sample_log.to_lowercase().contains("login")
            || request.sample_log.to_lowercase().contains("auth")
        {
            "authentication".to_string()
        } else if request.sample_log.to_lowercase().contains("process")
            || request.sample_log.to_lowercase().contains("exec")
        {
            "process_activity".to_string()
        } else if request.sample_log.to_lowercase().contains("network")
            || request.sample_log.to_lowercase().contains("connection")
        {
            "network_activity".to_string()
        } else if request.sample_log.to_lowercase().contains("file")
            || request.sample_log.to_lowercase().contains("path")
        {
            "file_activity".to_string()
        } else {
            "unknown".to_string()
        }
    };

    let mappings = vec![
        FieldMapping {
            source_field: "timestamp".to_string(),
            ocsf_field: "time".to_string(),
            transformation: Some("Convert to ISO 8601 format".to_string()),
        },
        FieldMapping {
            source_field: "username".to_string(),
            ocsf_field: "user.name".to_string(),
            transformation: None,
        },
        FieldMapping {
            source_field: "user_id".to_string(),
            ocsf_field: "user.uid".to_string(),
            transformation: None,
        },
    ];

    let recommendation = MappingRecommendation {
        suggested_event_class: event_class.clone(),
        confidence: "medium".to_string(),
        field_mappings: mappings,
        explanation: format!(
            "Based on log content analysis, suggested OCSF event class is '{event_class}'. \
             Map your fields to OCSF attributes for standardization. \
             Confidence is medium - please review and adjust as needed."
        ),
    };

    serde_json::to_string_pretty(&recommendation).map_err(|e| anyhow::anyhow!(e.to_string()))
}

/// List example OCSF events for learning
pub async fn list_event_examples(request: ListExamplesRequest) -> Result<String> {
    tracing::info!(
        "list_event_examples called: event_class={}",
        request.event_class
    );

    use crate::ocsf::EventExample;
    let mut examples = Vec::new();

    match request.event_class.as_str() {
        "authentication" => {
            if request.scenario.as_deref() == Some("failed_login") || request.scenario.is_none() {
                examples.push(EventExample::authentication_failure());
            }
            if request.scenario.as_deref() == Some("successful_login") || request.scenario.is_none()
            {
                examples.push(EventExample::authentication_success());
            }
        }
        "process_activity" => {
            if request.scenario.as_deref() == Some("process_start") || request.scenario.is_none() {
                examples.push(EventExample::process_start());
            }
        }
        _ => {
            return Err(anyhow::anyhow!(format!(
                "No examples available for event class '{}'. Available: authentication, process_activity",
                request.event_class
            )));
        }
    }

    serde_json::to_string_pretty(&examples).map_err(|e| anyhow::anyhow!(e.to_string()))
}
