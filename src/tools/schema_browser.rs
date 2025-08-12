use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::ocsf::{OcsfSchema, SchemaInfo};

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct BrowseSchemaRequest {
    #[schemars(description = "OCSF schema version (defaults to 1.7.0-dev)")]
    pub version: Option<String>,
    pub category: Option<String>,
    pub event_class: Option<String>,
    pub show_attributes: bool,
}

/// Browse OCSF schema categories, event classes, and attributes
pub async fn browse_ocsf_schema(request: BrowseSchemaRequest) -> Result<String> {
    let version = request.version.as_deref().unwrap_or("1.7.0-dev");

    tracing::info!(
        "browse_ocsf_schema called: version={}, category={:?}, event_class={:?}",
        version,
        request.category,
        request.event_class
    );

    let schema = OcsfSchema::load_version(version)
        .await
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;

    // If no filters, show all categories
    if request.category.is_none() && request.event_class.is_none() {
        let categories = schema.list_categories();
        let result = SchemaInfo {
            summary: format!(
                "OCSF v{} - {} categories available",
                schema.version,
                categories.len()
            ),
            categories: Some(categories),
            event_classes: None,
            attributes: None,
        };
        return serde_json::to_string_pretty(&result).map_err(|e| anyhow::anyhow!(e.to_string()));
    }

    // If category specified, show event classes in that category
    if let Some(cat_name) = request.category {
        let event_classes = schema.list_event_classes_for_category(&cat_name);
        let result = SchemaInfo {
            summary: format!(
                "Category '{}' contains {} event classes",
                cat_name,
                event_classes.len()
            ),
            categories: None,
            event_classes: Some(event_classes),
            attributes: None,
        };
        return serde_json::to_string_pretty(&result).map_err(|e| anyhow::anyhow!(e.to_string()));
    }

    // If event class specified, show its details
    if let Some(ec_name) = request.event_class {
        if let Some(ec) = schema.get_event_class(&ec_name) {
            let req_attrs = schema.get_required_attributes(&ec_name);

            let attributes = if request.show_attributes {
                Some(
                    req_attrs
                        .iter()
                        .map(|attr| crate::ocsf::AttributeSummary {
                            name: attr.clone(),
                            data_type: "string".to_string(),
                            description: format!("Required field for {}", ec.name),
                            required: true,
                        })
                        .collect(),
                )
            } else {
                None
            };

            let result = SchemaInfo {
                summary: format!(
                    "Event class '{}' (UID: {}) - {} required fields",
                    ec.name,
                    ec.uid,
                    req_attrs.len()
                ),
                categories: None,
                event_classes: None,
                attributes,
            };
            return serde_json::to_string_pretty(&result)
                .map_err(|e| anyhow::anyhow!(e.to_string()));
        }
    }

    Err(anyhow::anyhow!("Event class not found"))
}
