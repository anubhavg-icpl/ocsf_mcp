use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Validation result for OCSF events
#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationReport {
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationWarning>,
    pub event_class: Option<String>,
    pub summary: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
    pub error_type: ErrorType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ErrorType {
    MissingRequired,
    InvalidType,
    InvalidValue,
    UnknownField,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationWarning {
    pub field: String,
    pub message: String,
}

impl ValidationReport {
    pub fn new(is_valid: bool, event_class: Option<String>) -> Self {
        let summary = if is_valid {
            "Event is valid OCSF".to_string()
        } else {
            "Event has validation errors".to_string()
        };

        Self {
            is_valid,
            errors: Vec::new(),
            warnings: Vec::new(),
            event_class,
            summary,
        }
    }

    pub fn add_error(&mut self, field: String, message: String, error_type: ErrorType) {
        self.errors.push(ValidationError {
            field,
            message,
            error_type,
        });
        self.is_valid = false;
    }

    #[allow(dead_code)]
    pub fn add_warning(&mut self, field: String, message: String) {
        self.warnings.push(ValidationWarning { field, message });
    }
}

/// Validate an OCSF event against the schema
pub fn validate_event(event_json: &str) -> anyhow::Result<ValidationReport> {
    let event: Value = serde_json::from_str(event_json)?;

    let mut report = ValidationReport::new(true, None);

    // Check for metadata
    if let Some(metadata) = event.get("metadata") {
        // Validate metadata fields
        if let Some(event_class) = metadata.get("event_class").and_then(|v| v.as_str()) {
            report.event_class = Some(event_class.to_string());
        } else {
            report.add_error(
                "metadata.event_class".to_string(),
                "Missing event_class in metadata".to_string(),
                ErrorType::MissingRequired,
            );
        }

        if metadata.get("version").is_none() {
            report.add_error(
                "metadata.version".to_string(),
                "Missing version in metadata".to_string(),
                ErrorType::MissingRequired,
            );
        }
    } else {
        report.add_error(
            "metadata".to_string(),
            "Missing metadata field".to_string(),
            ErrorType::MissingRequired,
        );
    }

    // Check for required time field
    if event.get("time").is_none() {
        report.add_error(
            "time".to_string(),
            "Missing required 'time' field".to_string(),
            ErrorType::MissingRequired,
        );
    }

    // Generate summary
    if report.is_valid {
        report.summary = format!(
            "Valid OCSF event of class '{}'",
            report
                .event_class
                .as_ref()
                .unwrap_or(&"unknown".to_string())
        );
    } else {
        report.summary = format!("Validation failed with {} error(s)", report.errors.len());
    }

    Ok(report)
}
