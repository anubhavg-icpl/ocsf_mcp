use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// OCSF Schema representation (v1.7.0-dev format)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcsfSchema {
    pub version: String,
    #[serde(default)]
    pub classes: HashMap<String, EventClass>,
    #[serde(default)]
    pub objects: HashMap<String, Object>,
    #[serde(default)]
    pub types: HashMap<String, TypeDef>,
    #[serde(default)]
    pub dictionary_attributes: HashMap<String, Attribute>,
}

/// OCSF Event Class (from schema JSON)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventClass {
    #[serde(default)]
    pub uid: u32,
    pub name: String,
    #[serde(default)]
    pub caption: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub attributes: HashMap<String, Attribute>,
    #[serde(default)]
    pub extends: Option<String>,
}

/// OCSF Object definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Object {
    pub name: String,
    #[serde(default)]
    pub caption: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub attributes: HashMap<String, Attribute>,
    #[serde(default)]
    pub extends: Option<String>,
}

/// OCSF Attribute definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    #[serde(default)]
    pub caption: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub data_type: Option<String>,
    #[serde(default)]
    pub requirement: Option<String>,
    #[serde(default)]
    pub type_name: Option<String>,
}

/// OCSF Type definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeDef {
    #[serde(default)]
    pub caption: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

/// Schema browsing result
#[derive(Debug, Serialize, Deserialize)]
pub struct SchemaInfo {
    pub summary: String,
    pub categories: Option<Vec<CategorySummary>>,
    pub event_classes: Option<Vec<EventClassSummary>>,
    pub attributes: Option<Vec<AttributeSummary>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategorySummary {
    pub name: String,
    pub description: String,
    pub event_count: usize,
    pub event_classes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventClassSummary {
    pub uid: u32,
    pub name: String,
    pub caption: String,
    pub description: String,
    pub category: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttributeSummary {
    pub name: String,
    pub data_type: String,
    pub description: String,
    pub required: bool,
}

impl OcsfSchema {
    /// Load OCSF schema from file or fallback to embedded (defaults to v1.7.0-dev)
    #[allow(dead_code)]
    pub async fn load() -> anyhow::Result<Self> {
        Self::load_version("1.7.0-dev").await
    }

    /// List all available OCSF schema versions
    pub fn list_versions() -> anyhow::Result<Vec<String>> {
        let schema_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/ocsf-schema");

        if !schema_dir.exists() {
            return Ok(vec!["1.7.0-dev".to_string()]); // Fallback to embedded
        }

        let mut versions = Vec::new();
        for entry in std::fs::read_dir(schema_dir)? {
            let entry = entry?;
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.ends_with(".json") {
                    versions.push(name.trim_end_matches(".json").to_string());
                }
            }
        }

        versions.sort();
        Ok(versions)
    }

    /// Get the newest stable OCSF version (excludes dev/alpha/beta/rc)
    pub fn get_newest_stable_version() -> anyhow::Result<String> {
        let versions = Self::list_versions()?;
        let stable: Vec<_> = versions
            .iter()
            .filter(|v| {
                let v_lower = v.to_lowercase();
                !v_lower.contains("dev")
                    && !v_lower.contains("alpha")
                    && !v_lower.contains("beta")
                    && !v_lower.contains("rc")
            })
            .collect();

        stable
            .last()
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow::anyhow!("No stable OCSF versions found"))
    }

    /// Load a specific OCSF schema version
    pub async fn load_version(version: &str) -> anyhow::Result<Self> {
        let schema_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(format!("data/ocsf-schema/{version}.json"));

        if schema_path.exists() {
            tracing::info!("Loading OCSF schema v{} from {:?}", version, schema_path);
            let content = tokio::fs::read_to_string(&schema_path).await?;
            let schema: OcsfSchema = serde_json::from_str(&content)?;
            tracing::info!(
                "Loaded OCSF v{} - {} classes, {} objects",
                schema.version,
                schema.classes.len(),
                schema.objects.len()
            );
            Ok(schema)
        } else {
            tracing::warn!(
                "Schema version {} not found at {:?}, using minimal embedded schema",
                version,
                schema_path
            );
            Ok(Self::minimal_schema())
        }
    }

    /// Create a minimal embedded schema for fallback
    fn minimal_schema() -> Self {
        let mut classes = HashMap::new();

        // Authentication event class (IAM category)
        classes.insert(
            "authentication".to_string(),
            EventClass {
                uid: 3002,
                name: "authentication".to_string(),
                caption: Some("Authentication".to_string()),
                description: Some(
                    "User authentication events (login, logout, failed attempts)".to_string(),
                ),
                category: "iam".to_string(),
                attributes: HashMap::new(),
                extends: None,
            },
        );

        // Process Activity event class (System category)
        classes.insert(
            "process_activity".to_string(),
            EventClass {
                uid: 1007,
                name: "process_activity".to_string(),
                caption: Some("Process Activity".to_string()),
                description: Some("Process lifecycle events (start, stop, injection)".to_string()),
                category: "system".to_string(),
                attributes: HashMap::new(),
                extends: None,
            },
        );

        // File Activity event class (System category)
        classes.insert(
            "file_activity".to_string(),
            EventClass {
                uid: 1001,
                name: "file_activity".to_string(),
                caption: Some("File Activity".to_string()),
                description: Some("File system operations".to_string()),
                category: "system".to_string(),
                attributes: HashMap::new(),
                extends: None,
            },
        );

        // Network Activity event class (Network category)
        classes.insert(
            "network_activity".to_string(),
            EventClass {
                uid: 4001,
                name: "network_activity".to_string(),
                caption: Some("Network Activity".to_string()),
                description: Some("Network connections and traffic".to_string()),
                category: "network".to_string(),
                attributes: HashMap::new(),
                extends: None,
            },
        );

        Self {
            version: "1.7.0-dev".to_string(),
            classes,
            objects: HashMap::new(),
            types: HashMap::new(),
            dictionary_attributes: HashMap::new(),
        }
    }

    pub fn get_event_class(&self, name: &str) -> Option<&EventClass> {
        self.classes.get(name)
    }

    pub fn list_categories(&self) -> Vec<CategorySummary> {
        // Group classes by category
        let mut category_map: HashMap<String, Vec<String>> = HashMap::new();

        for (name, class) in &self.classes {
            if name == "base_event" {
                continue; // Skip base event
            }
            category_map
                .entry(class.category.clone())
                .or_default()
                .push(name.clone());
        }

        category_map
            .into_iter()
            .map(|(name, event_classes)| CategorySummary {
                description: get_category_description(&name),
                event_count: event_classes.len(),
                name: name.clone(),
                event_classes,
            })
            .collect()
    }

    pub fn list_event_classes_for_category(&self, category: &str) -> Vec<EventClassSummary> {
        self.classes
            .values()
            .filter(|ec| ec.category == category && ec.name != "base_event")
            .map(|ec| EventClassSummary {
                uid: ec.uid,
                name: ec.name.clone(),
                caption: ec.caption.clone().unwrap_or_else(|| ec.name.clone()),
                description: ec
                    .description
                    .clone()
                    .unwrap_or_else(|| "No description available".to_string()),
                category: ec.category.clone(),
            })
            .collect()
    }

    #[allow(dead_code)]
    pub fn list_all_event_classes(&self) -> Vec<EventClassSummary> {
        self.classes
            .values()
            .filter(|ec| ec.name != "base_event")
            .map(|ec| EventClassSummary {
                uid: ec.uid,
                name: ec.name.clone(),
                caption: ec.caption.clone().unwrap_or_else(|| ec.name.clone()),
                description: ec
                    .description
                    .clone()
                    .unwrap_or_else(|| "No description available".to_string()),
                category: ec.category.clone(),
            })
            .collect()
    }

    pub fn get_required_attributes(&self, event_class: &str) -> Vec<String> {
        self.classes
            .get(event_class)
            .map(|ec| {
                ec.attributes
                    .iter()
                    .filter(|(_, attr)| {
                        attr.requirement.as_deref() == Some("required")
                            || attr.requirement.as_deref() == Some("recommended")
                    })
                    .map(|(name, _)| name.clone())
                    .collect()
            })
            .unwrap_or_default()
    }
}

fn get_category_description(category: &str) -> String {
    match category {
        "system" => "System Activity - Operating system and device-level events".to_string(),
        "findings" => {
            "Findings - Security findings from scanning, detection, and analysis".to_string()
        }
        "iam" => {
            "Identity & Access Management - Authentication, authorization, and account management"
                .to_string()
        }
        "network" => "Network Activity - Network connections and traffic".to_string(),
        "discovery" => "Discovery - Resource and asset discovery".to_string(),
        "application" => "Application Activity - Application-specific events".to_string(),
        "remediation" => "Remediation - Security remediation activities".to_string(),
        "other" => "Other - Miscellaneous events".to_string(),
        _ => format!("Category: {category}"),
    }
}
