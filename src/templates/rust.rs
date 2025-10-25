use anyhow::Result;

use super::{CodeArtifacts, CodeFile};

/// Generate Rust code for OCSF logging
pub fn generate(
    event_classes: Vec<String>,
    _framework: Option<String>,
    include_helpers: bool,
) -> Result<CodeArtifacts> {
    let mut files = Vec::new();

    // Core OCSF module
    files.push(generate_core_module());

    // Event builders if requested
    if include_helpers {
        files.push(generate_builder_module(&event_classes));
    }

    // Event-specific modules for each class
    for event_class in &event_classes {
        files.push(generate_event_module(event_class)?);
    }

    Ok(CodeArtifacts {
        summary: format!(
            "Generated Rust OCSF logging code for {} event classes with {} files",
            event_classes.len(),
            files.len()
        ),
        language: "rust".to_string(),
        files,
    })
}

fn generate_core_module() -> CodeFile {
    let content = r#"//! OCSF Core Event Structures
//!
//! This module provides the foundational types for OCSF events.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Core OCSF Event structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcsfEvent {
    pub metadata: EventMetadata,
    #[serde(flatten)]
    pub fields: HashMap<String, Value>,
}

/// Event metadata following OCSF v1.3.0
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMetadata {
    pub version: String,
    pub event_class: String,
    pub category_uid: u32,
    pub class_uid: u32,
    pub uid: String,
}

impl OcsfEvent {
    /// Create a new OCSF event
    pub fn new(event_class: &str, class_uid: u32, category_uid: u32) -> Self {
        Self {
            metadata: EventMetadata {
                version: "1.7.0-dev".to_string(),
                event_class: event_class.to_string(),
                category_uid,
                class_uid,
                uid: uuid::Uuid::new_v4().to_string(),
            },
            fields: HashMap::new(),
        }
    }

    /// Add a field to the event
    pub fn add_field(&mut self, key: impl Into<String>, value: Value) {
        self.fields.insert(key.into(), value);
    }

    /// Serialize to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Serialize to compact JSON
    pub fn to_json_compact(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_event() {
        let event = OcsfEvent::new("authentication", 3002, 3);
        assert_eq!(event.metadata.event_class, "authentication");
        assert_eq!(event.metadata.class_uid, 3002);
    }
}
"#;

    CodeFile {
        filename: "ocsf_core.rs".to_string(),
        content: content.to_string(),
        description: "Core OCSF event structure with metadata and field handling".to_string(),
    }
}

fn generate_builder_module(event_classes: &[String]) -> CodeFile {
    let content = format!(
        r#"//! OCSF Event Builders
//!
//! Provides builder pattern for constructing OCSF events.

use super::ocsf_core::OcsfEvent;
use serde_json::{{json, Value}};

/// Fluent builder for OCSF events
pub struct EventBuilder {{
    event: OcsfEvent,
}}

impl EventBuilder {{
    /// Create a new event builder
    pub fn new(event_class: &str, class_uid: u32, category_uid: u32) -> Self {{
        Self {{
            event: OcsfEvent::new(event_class, class_uid, category_uid),
        }}
    }}

    /// Add a field with any JSON-serializable value
    pub fn field(mut self, key: &str, value: Value) -> Self {{
        self.event.add_field(key, value);
        self
    }}

    /// Add a string field
    pub fn string_field(self, key: &str, value: impl Into<String>) -> Self {{
        self.field(key, json!(value.into()))
    }}

    /// Add a numeric field
    pub fn num_field(self, key: &str, value: impl Into<i64>) -> Self {{
        self.field(key, json!(value.into()))
    }}

    /// Add timestamp (current time)
    pub fn with_timestamp(self) -> Self {{
        self.field("time", json!(chrono::Utc::now().to_rfc3339()))
    }}

    /// Build the final event
    pub fn build(self) -> OcsfEvent {{
        self.event
    }}
}}

// Supported event classes: {}

#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn test_builder_pattern() {{
        let event = EventBuilder::new("authentication", 3002, 3)
            .string_field("user", "john.doe")
            .string_field("status", "success")
            .with_timestamp()
            .build();

        assert!(event.fields.contains_key("user"));
        assert!(event.fields.contains_key("time"));
    }}
}}
"#,
        event_classes.join(", ")
    );

    CodeFile {
        filename: "event_builder.rs".to_string(),
        content,
        description: "Fluent builder pattern for constructing OCSF events easily".to_string(),
    }
}

fn generate_event_module(event_class: &str) -> Result<CodeFile> {
    let (class_uid, category_uid, description) = match event_class {
        "authentication" => (3002, 3, "User authentication events (login, logout, etc.)"),
        "process_activity" => (1007, 1, "Process lifecycle events (start, stop, etc.)"),
        "file_activity" => (1001, 1, "File system operations"),
        "network_activity" => (4001, 4, "Network connections and traffic"),
        _ => {
            return Err(anyhow::anyhow!(format!(
                "Unknown event class: {event_class}"
            )));
        }
    };

    let module_name = event_class.replace("_", "");
    let struct_name = event_class
        .split('_')
        .map(|s| {
            let mut chars = s.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<String>();

    let content = format!(
        r#"//! {struct_name} OCSF Events
//!
//! {description}

use super::{{ocsf_core::OcsfEvent, event_builder::EventBuilder}};
use serde_json::{{json, Value}};

/// {struct_name} event (Class UID: {class_uid})
pub struct {struct_name} {{
    event: OcsfEvent,
}}

impl {struct_name} {{
    /// Create a new {struct_name} event
    pub fn new() -> Self {{
        Self {{
            event: OcsfEvent::new("{event_class}", {class_uid}, {category_uid}),
        }}
    }}

    /// Get a builder for this event type
    pub fn builder() -> EventBuilder {{
        EventBuilder::new("{event_class}", {class_uid}, {category_uid})
    }}

    /// Add a field to the event
    pub fn add_field(&mut self, key: impl Into<String>, value: Value) {{
        self.event.add_field(key, value);
    }}

    /// Convert to JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {{
        self.event.to_json()
    }}

    /// Get the inner event
    pub fn into_inner(self) -> OcsfEvent {{
        self.event
    }}
}}

impl Default for {struct_name} {{
    fn default() -> Self {{
        Self::new()
    }}
}}

#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn test_{module_name}_creation() {{
        let event = {struct_name}::new();
        assert_eq!(event.event.metadata.class_uid, {class_uid});
    }}
}}
"#
    );

    Ok(CodeFile {
        filename: format!("{module_name}.rs"),
        content,
        description: format!("OCSF {event_class} event implementation"),
    })
}
