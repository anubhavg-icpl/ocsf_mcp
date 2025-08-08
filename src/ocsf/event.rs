use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// Generic OCSF Event structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcsfEvent {
    /// Metadata about the event class
    pub metadata: EventMetadata,
    /// Event-specific fields
    #[serde(flatten)]
    pub fields: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMetadata {
    pub version: String,
    pub product: Option<ProductInfo>,
    pub uid: String,
    pub event_class: String,
    pub category_uid: u32,
    pub class_uid: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductInfo {
    pub name: String,
    pub vendor_name: String,
    pub version: Option<String>,
}

impl OcsfEvent {
    pub fn new(event_class: &str, class_uid: u32, category_uid: u32) -> Self {
        use uuid::Uuid;
        Self {
            metadata: EventMetadata {
                version: "1.7.0-dev".to_string(),
                product: None,
                uid: Uuid::new_v4().to_string(),
                event_class: event_class.to_string(),
                category_uid,
                class_uid,
            },
            fields: Map::new(),
        }
    }

    pub fn set_field(&mut self, key: String, value: Value) {
        self.fields.insert(key, value);
    }

    pub fn to_json(&self) -> anyhow::Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
}

/// Event examples for learning
#[derive(Debug, Serialize, Deserialize)]
pub struct EventExample {
    pub event_class: String,
    pub scenario: String,
    pub description: String,
    pub json: String,
}

impl EventExample {
    pub fn authentication_success() -> Self {
        let json = r#"{
  "metadata": {
    "version": "1.7.0-dev",
    "event_class": "authentication",
    "category_uid": 3,
    "class_uid": 3002,
    "uid": "123e4567-e89b-12d3-a456-426614174000"
  },
  "time": "2025-01-15T10:30:00Z",
  "user": {
    "name": "john.doe",
    "uid": "1001"
  },
  "auth_protocol": "OAuth2",
  "status": "success",
  "activity_id": 1,
  "activity_name": "Logon"
}"#;

        Self {
            event_class: "authentication".to_string(),
            scenario: "successful_login".to_string(),
            description: "User successfully logged in using OAuth2".to_string(),
            json: json.to_string(),
        }
    }

    pub fn authentication_failure() -> Self {
        let json = r#"{
  "metadata": {
    "version": "1.7.0-dev",
    "event_class": "authentication",
    "category_uid": 3,
    "class_uid": 3002,
    "uid": "223e4567-e89b-12d3-a456-426614174001"
  },
  "time": "2025-01-15T10:35:00Z",
  "user": {
    "name": "attacker",
    "uid": "unknown"
  },
  "auth_protocol": "LDAP",
  "status": "failure",
  "status_detail": "Invalid credentials",
  "activity_id": 1,
  "activity_name": "Logon",
  "severity": "medium"
}"#;

        Self {
            event_class: "authentication".to_string(),
            scenario: "failed_login".to_string(),
            description: "Failed login attempt with invalid credentials".to_string(),
            json: json.to_string(),
        }
    }

    pub fn process_start() -> Self {
        let json = r#"{
  "metadata": {
    "version": "1.7.0-dev",
    "event_class": "process_activity",
    "category_uid": 1,
    "class_uid": 1007,
    "uid": "323e4567-e89b-12d3-a456-426614174002"
  },
  "time": "2025-01-15T11:00:00Z",
  "process": {
    "name": "nginx",
    "pid": 1234,
    "uid": "501",
    "cmd_line": "/usr/sbin/nginx -c /etc/nginx/nginx.conf"
  },
  "activity_id": 1,
  "activity_name": "Launch",
  "parent_process": {
    "name": "systemd",
    "pid": 1
  }
}"#;

        Self {
            event_class: "process_activity".to_string(),
            scenario: "process_start".to_string(),
            description: "Process started by systemd".to_string(),
            json: json.to_string(),
        }
    }
}
