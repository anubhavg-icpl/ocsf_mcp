use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::templates;

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct GenerateCodeRequest {
    pub language: String,
    pub event_classes: String,
    pub framework: Option<String>,
    pub include_helpers: bool,
}

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct CodeArtifacts {
    pub summary: String,
    pub language: String,
    pub files: Vec<CodeFile>,
}

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct CodeFile {
    pub filename: String,
    pub content: String,
    pub description: String,
}

/// Generate OCSF logging code for a specific language/framework
pub async fn generate_logging_code(request: GenerateCodeRequest) -> Result<String> {
    tracing::info!(
        "generate_logging_code called: language={}",
        request.language
    );

    let classes: Vec<String> =
        serde_json::from_str(&request.event_classes).map_err(|e| anyhow::anyhow!(e.to_string()))?;

    let artifacts = match request.language.to_lowercase().as_str() {
        "rust" => templates::rust::generate(classes, request.framework, request.include_helpers)?,
        "python" => {
            templates::python::generate(classes, request.framework, request.include_helpers)?
        }
        "javascript" | "js" => {
            templates::javascript::generate(classes, request.framework, request.include_helpers)?
        }
        _ => {
            return Err(anyhow::anyhow!(format!(
                "Language '{}' not yet supported. Available: rust, python, javascript",
                request.language
            )))
        }
    };

    serde_json::to_string_pretty(&artifacts).map_err(|e| anyhow::anyhow!(e.to_string()))
}
