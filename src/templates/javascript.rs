use anyhow::Result;

use super::{CodeArtifacts, CodeFile};

/// Generate JavaScript/TypeScript code for OCSF logging
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

    // index.js for exports
    files.push(generate_index_module(&event_classes));

    // package.json
    files.push(generate_package_json());

    Ok(CodeArtifacts {
        summary: format!(
            "Generated JavaScript OCSF logging code for {} event classes with {} files",
            event_classes.len(),
            files.len()
        ),
        language: "javascript".to_string(),
        files,
    })
}

fn generate_core_module() -> CodeFile {
    let content = r#"/**
 * OCSF Core Event Structures
 *
 * This module provides the foundational types for OCSF events.
 */

const { v4: uuidv4 } = require('uuid');

/**
 * Core OCSF Event class following v1.3.0
 */
class OcsfEvent {
  /**
   * Create a new OCSF event
   * @param {string} eventClass - The OCSF event class name
   * @param {number} classUid - The OCSF class UID
   * @param {number} categoryUid - The OCSF category UID
   */
  constructor(eventClass, classUid, categoryUid) {
    this.metadata = {
      version: '1.7.0-dev',
      event_class: eventClass,
      category_uid: categoryUid,
      class_uid: classUid,
      uid: uuidv4()
    };
    this.fields = {};
  }

  /**
   * Add a field to the event
   * @param {string} key - Field name
   * @param {any} value - Field value
   * @returns {OcsfEvent} This event (for chaining)
   */
  addField(key, value) {
    this.fields[key] = value;
    return this;
  }

  /**
   * Convert to plain object
   * @returns {Object} Event as plain object
   */
  toObject() {
    return {
      metadata: this.metadata,
      ...this.fields
    };
  }

  /**
   * Serialize to JSON string
   * @param {boolean} pretty - Pretty print the JSON
   * @returns {string} JSON string
   */
  toJSON(pretty = true) {
    return JSON.stringify(this.toObject(), null, pretty ? 2 : 0);
  }
}

module.exports = { OcsfEvent };
"#;

    CodeFile {
        filename: "ocsf-core.js".to_string(),
        content: content.to_string(),
        description: "Core OCSF event class with metadata and field handling".to_string(),
    }
}

fn generate_builder_module(event_classes: &[String]) -> CodeFile {
    let content = format!(
        r#"/**
 * OCSF Event Builders
 *
 * Provides builder pattern for constructing OCSF events.
 */

const {{ OcsfEvent }} = require('./ocsf-core');

/**
 * Fluent builder for OCSF events
 */
class EventBuilder {{
  /**
   * Create a new event builder
   * @param {{string}} eventClass - The OCSF event class name
   * @param {{number}} classUid - The OCSF class UID
   * @param {{number}} categoryUid - The OCSF category UID
   */
  constructor(eventClass, classUid, categoryUid) {{
    this.event = new OcsfEvent(eventClass, classUid, categoryUid);
  }}

  /**
   * Add a field (chainable)
   * @param {{string}} key - Field name
   * @param {{any}} value - Field value
   * @returns {{EventBuilder}} This builder
   */
  field(key, value) {{
    this.event.addField(key, value);
    return this;
  }}

  /**
   * Add a string field (chainable)
   * @param {{string}} key - Field name
   * @param {{string}} value - Field value
   * @returns {{EventBuilder}} This builder
   */
  stringField(key, value) {{
    return this.field(key, String(value));
  }}

  /**
   * Add a numeric field (chainable)
   * @param {{string}} key - Field name
   * @param {{number}} value - Field value
   * @returns {{EventBuilder}} This builder
   */
  numField(key, value) {{
    return this.field(key, Number(value));
  }}

  /**
   * Add current timestamp (chainable)
   * @returns {{EventBuilder}} This builder
   */
  withTimestamp() {{
    return this.field('time', new Date().toISOString());
  }}

  /**
   * Build the final event
   * @returns {{OcsfEvent}} The constructed event
   */
  build() {{
    return this.event;
  }}
}}

// Supported event classes: {}

module.exports = {{ EventBuilder }};
"#,
        event_classes.join(", ")
    );

    CodeFile {
        filename: "event-builder.js".to_string(),
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
            )));
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
        r#"/**
 * {class_name}
 *
 * {description}
 */

const {{ OcsfEvent }} = require('./ocsf-core');
const {{ EventBuilder }} = require('./event-builder');

/**
 * {class_name} event (Class UID: {class_uid})
 */
class {class_name} {{
  static CLASS_UID = {class_uid};
  static CATEGORY_UID = {category_uid};

  /**
   * Create a new {class_name} event
   */
  constructor() {{
    this.event = new OcsfEvent('{event_class}', {class_name}.CLASS_UID, {class_name}.CATEGORY_UID);
  }}

  /**
   * Get a builder for this event type
   * @returns {{EventBuilder}} A new event builder
   */
  static builder() {{
    return new EventBuilder('{event_class}', {class_name}.CLASS_UID, {class_name}.CATEGORY_UID);
  }}

  /**
   * Add a field to the event (chainable)
   * @param {{string}} key - Field name
   * @param {{any}} value - Field value
   * @returns {{{class_name}}} This event
   */
  addField(key, value) {{
    this.event.addField(key, value);
    return this;
  }}

  /**
   * Convert to JSON
   * @param {{boolean}} pretty - Pretty print the JSON
   * @returns {{string}} JSON string
   */
  toJSON(pretty = true) {{
    return this.event.toJSON(pretty);
  }}

  /**
   * Convert to plain object
   * @returns {{Object}} Event as plain object
   */
  toObject() {{
    return this.event.toObject();
  }}
}}

module.exports = {{ {class_name} }};
"#
    );

    Ok(CodeFile {
        filename: format!("{}.js", event_class.replace("_", "-")),
        content,
        description: format!("OCSF {event_class} event implementation"),
    })
}

fn generate_index_module(event_classes: &[String]) -> CodeFile {
    let requires: Vec<String> = event_classes
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
            let file_name = ec.replace("_", "-");
            format!("const {{ {class_name} }} = require('./{file_name}');")
        })
        .collect();

    let exports: Vec<String> = event_classes
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
            format!("  {class_name}")
        })
        .collect();

    let content = format!(
        r#"/**
 * OCSF Event Package
 *
 * Provides OCSF-compliant event logging for JavaScript/Node.js applications.
 */

const {{ OcsfEvent }} = require('./ocsf-core');
const {{ EventBuilder }} = require('./event-builder');
{}

module.exports = {{
  OcsfEvent,
  EventBuilder,
{}
}};
"#,
        requires.join("\n"),
        exports.join(",\n")
    );

    CodeFile {
        filename: "index.js".to_string(),
        content,
        description: "Main entry point with exports".to_string(),
    }
}

fn generate_package_json() -> CodeFile {
    let content = r#"{
  "name": "ocsf-events",
  "version": "1.7.0-dev",
  "description": "OCSF-compliant event logging for Node.js",
  "main": "index.js",
  "scripts": {
    "test": "jest"
  },
  "keywords": [
    "ocsf",
    "security",
    "logging",
    "events"
  ],
  "dependencies": {
    "uuid": "^9.0.0"
  },
  "devDependencies": {
    "jest": "^29.0.0"
  },
  "engines": {
    "node": ">=14.0.0"
  },
  "license": "Apache-2.0"
}
"#;

    CodeFile {
        filename: "package.json".to_string(),
        content: content.to_string(),
        description: "NPM package configuration".to_string(),
    }
}
