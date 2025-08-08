// MCP Tools for OCSF Server
// Each tool is in its own module for better organization

pub mod code_generator;
pub mod event_generator;
pub mod mapper;
pub mod schema_browser;
pub mod validator;

// Re-export for convenience
pub use code_generator::{generate_logging_code as generate_code_impl, GenerateCodeRequest};
pub use event_generator::{generate_ocsf_event as generate_event_impl, GenerateEventRequest};
pub use mapper::{
    list_event_examples as list_examples_impl, map_custom_to_ocsf as map_custom_impl,
    ListExamplesRequest, MapCustomRequest,
};
pub use schema_browser::{browse_ocsf_schema as browse_schema_impl, BrowseSchemaRequest};
pub use validator::{validate_ocsf_event as validate_event_impl, ValidateEventRequest};

use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::*,
    tool, tool_handler, tool_router, ErrorData as McpError, ServerHandler,
};

/// OCSF MCP Server - implements ServerHandler with tool routing
#[derive(Debug, Clone)]
pub struct OcsfServer {
    tool_router: ToolRouter<Self>,
}

impl OcsfServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }
}

impl Default for OcsfServer {
    fn default() -> Self {
        Self::new()
    }
}

// Tool implementations
#[tool_router]
impl OcsfServer {
    #[tool(description = "Browse OCSF schema categories, event classes, and attributes")]
    async fn browse_ocsf_schema(
        &self,
        Parameters(request): Parameters<BrowseSchemaRequest>,
    ) -> Result<CallToolResult, McpError> {
        match browse_schema_impl(request).await {
            Ok(result) => Ok(CallToolResult::success(vec![Content::text(result)])),
            Err(e) => Err(McpError::internal_error(
                "browse_schema_error",
                Some(serde_json::json!({"error": e.to_string()})),
            )),
        }
    }

    #[tool(description = "Generate a valid OCSF event JSON from parameters")]
    async fn generate_ocsf_event(
        &self,
        Parameters(request): Parameters<GenerateEventRequest>,
    ) -> Result<CallToolResult, McpError> {
        match generate_event_impl(request).await {
            Ok(result) => Ok(CallToolResult::success(vec![Content::text(result)])),
            Err(e) => Err(McpError::internal_error(
                "generate_event_error",
                Some(serde_json::json!({"error": e.to_string()})),
            )),
        }
    }

    #[tool(description = "Validate an OCSF event JSON against the schema")]
    async fn validate_ocsf_event(
        &self,
        Parameters(request): Parameters<ValidateEventRequest>,
    ) -> Result<CallToolResult, McpError> {
        match validate_event_impl(request).await {
            Ok(result) => Ok(CallToolResult::success(vec![Content::text(result)])),
            Err(e) => Err(McpError::internal_error(
                "validate_event_error",
                Some(serde_json::json!({"error": e.to_string()})),
            )),
        }
    }

    #[tool(description = "Generate OCSF logging code for a specific language/framework")]
    async fn generate_logging_code(
        &self,
        Parameters(request): Parameters<GenerateCodeRequest>,
    ) -> Result<CallToolResult, McpError> {
        match generate_code_impl(request).await {
            Ok(result) => Ok(CallToolResult::success(vec![Content::text(result)])),
            Err(e) => Err(McpError::internal_error(
                "generate_code_error",
                Some(serde_json::json!({"error": e.to_string()})),
            )),
        }
    }

    #[tool(description = "Map custom log format to OCSF event class")]
    async fn map_custom_to_ocsf(
        &self,
        Parameters(request): Parameters<MapCustomRequest>,
    ) -> Result<CallToolResult, McpError> {
        match map_custom_impl(request).await {
            Ok(result) => Ok(CallToolResult::success(vec![Content::text(result)])),
            Err(e) => Err(McpError::internal_error(
                "map_custom_error",
                Some(serde_json::json!({"error": e.to_string()})),
            )),
        }
    }

    #[tool(description = "List example OCSF events for learning")]
    async fn list_event_examples(
        &self,
        Parameters(request): Parameters<ListExamplesRequest>,
    ) -> Result<CallToolResult, McpError> {
        match list_examples_impl(request).await {
            Ok(result) => Ok(CallToolResult::success(vec![Content::text(result)])),
            Err(e) => Err(McpError::internal_error(
                "list_examples_error",
                Some(serde_json::json!({"error": e.to_string()})),
            )),
        }
    }
}

// ServerHandler implementation
#[tool_handler]
impl ServerHandler for OcsfServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "MCP server for implementing OCSF-based logging in any application. \
                 Provides schema browsing, event generation, validation, and code generation tools. \
                 Tools: browse_ocsf_schema, generate_ocsf_event, validate_ocsf_event, \
                 generate_logging_code, map_custom_to_ocsf, list_event_examples."
                    .to_string(),
            ),
        }
    }
}
