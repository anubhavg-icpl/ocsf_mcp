use anyhow::Result;

use super::{CodeArtifacts, CodeFile};

/// Generate Python code for OCSF logging
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

    // __init__.py for package
    files.push(generate_init_module(&event_classes));

    Ok(CodeArtifacts {
        summary: format!(
            "Generated Python OCSF logging code for {} event classes with {} files",
            event_classes.len(),
            files.len()
        ),
        language: "python".to_string(),
        files,
    })
}

fn generate_core_module() -> CodeFile {
    let content = r#"""OCSF Core Event Structures

This module provides the foundational types for OCSF events.
"""

import json
import uuid
from datetime import datetime
from typing import Dict, Any, Optional


class OcsfEvent:
    """Core OCSF Event structure following v1.3.0"""

    def __init__(self, event_class: str, class_uid: int, category_uid: int):
        """
        Create a new OCSF event.

        Args:
            event_class: The OCSF event class name
            class_uid: The OCSF class UID
            category_uid: The OCSF category UID
        """
        self.metadata = {
            "version": "1.7.0-dev",
            "event_class": event_class,
            "category_uid": category_uid,
            "class_uid": class_uid,
            "uid": str(uuid.uuid4()),
        }
        self.fields: Dict[str, Any] = {}

    def add_field(self, key: str, value: Any) -> 'OcsfEvent':
        """Add a field to the event (chainable)"""
        self.fields[key] = value
        return self

    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary"""
        return {"metadata": self.metadata, **self.fields}

    def to_json(self, pretty: bool = True) -> str:
        """Serialize to JSON string"""
        indent = 2 if pretty else None
        return json.dumps(self.to_dict(), indent=indent)

    def __repr__(self) -> str:
        return f"OcsfEvent(class={self.metadata['event_class']}, uid={self.metadata['uid']})"
"#;

    CodeFile {
        filename: "ocsf_core.py".to_string(),
        content: content.to_string(),
        description: "Core OCSF event class with metadata and field handling".to_string(),
    }
}

fn generate_builder_module(event_classes: &[String]) -> CodeFile {
    let content = format!(
        r#"""OCSF Event Builders

Provides builder pattern for constructing OCSF events.
"""

from datetime import datetime
from typing import Any, Optional
from .ocsf_core import OcsfEvent


class EventBuilder:
    """Fluent builder for OCSF events"""

    def __init__(self, event_class: str, class_uid: int, category_uid: int):
        """Create a new event builder"""
        self.event = OcsfEvent(event_class, class_uid, category_uid)

    def field(self, key: str, value: Any) -> 'EventBuilder':
        """Add a field with any value (chainable)"""
        self.event.add_field(key, value)
        return self

    def string_field(self, key: str, value: str) -> 'EventBuilder':
        """Add a string field (chainable)"""
        return self.field(key, value)

    def num_field(self, key: str, value: int) -> 'EventBuilder':
        """Add a numeric field (chainable)"""
        return self.field(key, value)

    def with_timestamp(self) -> 'EventBuilder':
        """Add current timestamp (chainable)"""
        return self.field("time", datetime.utcnow().isoformat() + "Z")

    def build(self) -> OcsfEvent:
        """Build the final event"""
        return self.event


# Supported event classes: {}
"#,
        event_classes.join(", ")
    );

    CodeFile {
        filename: "event_builder.py".to_string(),
        content,
        description: "Fluent builder pattern for constructing OCSF events".to_string(),
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
            )))
        }
    };

    let class_name = event_class
        .split('_')
        .map(|s| {
            let mut chars = s.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<String>()
        + "Event";

    let content = format!(
        r#"""{class_name}

{description}
"""

from typing import Any
from .ocsf_core import OcsfEvent
from .event_builder import EventBuilder


class {class_name}:
    """{class_name} event (Class UID: {class_uid})"""

    CLASS_UID = {class_uid}
    CATEGORY_UID = {category_uid}

    def __init__(self):
        """Create a new {class_name} event"""
        self.event = OcsfEvent("{event_class}", self.CLASS_UID, self.CATEGORY_UID)

    @classmethod
    def builder(cls) -> EventBuilder:
        """Get a builder for this event type"""
        return EventBuilder("{event_class}", cls.CLASS_UID, cls.CATEGORY_UID)

    def add_field(self, key: str, value: Any) -> '{class_name}':
        """Add a field to the event (chainable)"""
        self.event.add_field(key, value)
        return self

    def to_json(self, pretty: bool = True) -> str:
        """Convert to JSON"""
        return self.event.to_json(pretty=pretty)

    def to_dict(self) -> dict:
        """Convert to dictionary"""
        return self.event.to_dict()
"#
    );

    Ok(CodeFile {
        filename: format!("{event_class}.py"),
        content,
        description: format!("OCSF {event_class} event implementation"),
    })
}

fn generate_init_module(event_classes: &[String]) -> CodeFile {
    let imports: Vec<String> = event_classes
        .iter()
        .map(|ec| {
            let class_name = ec
                .split('_')
                .map(|s| {
                    let mut chars = s.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                    }
                })
                .collect::<String>()
                + "Event";
            format!("from .{ec} import {class_name}")
        })
        .collect();

    let all_exports: Vec<String> = event_classes
        .iter()
        .map(|ec| {
            let class_name = ec
                .split('_')
                .map(|s| {
                    let mut chars = s.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                    }
                })
                .collect::<String>()
                + "Event";
            format!("    \"{class_name}\"")
        })
        .collect();

    let content = format!(
        r#"""OCSF Event Package

Provides OCSF-compliant event logging for Python applications.
"""

from .ocsf_core import OcsfEvent
from .event_builder import EventBuilder
{}

__version__ = "1.7.0-dev"
__all__ = [
    "OcsfEvent",
    "EventBuilder",
{}
]
"#,
        imports.join("\n"),
        all_exports.join(",\n")
    );

    CodeFile {
        filename: "__init__.py".to_string(),
        content,
        description: "Package initialization with exports".to_string(),
    }
}
