// MCP Tools for OCSF Server
// Each tool is in its own module for better organization

pub mod code_generator;
pub mod docs_tool;
pub mod event_generator;
pub mod mapper;
pub mod schema_browser;
pub mod validator;
pub mod version_tools;

// Re-export for convenience
pub use code_generator::{GenerateCodeRequest, generate_logging_code as generate_code_impl};
pub use docs_tool::{ReadOcsfDocsRequest, read_ocsf_docs as read_docs_impl};
pub use event_generator::{GenerateEventRequest, generate_ocsf_event as generate_event_impl};
pub use mapper::{
    ListExamplesRequest, MapCustomRequest, list_event_examples as list_examples_impl,
    map_custom_to_ocsf as map_custom_impl,
};
pub use schema_browser::{BrowseSchemaRequest, browse_ocsf_schema as browse_schema_impl};
pub use validator::{ValidateEventRequest, validate_ocsf_event as validate_event_impl};
pub use version_tools::{
    GetNewestVersionRequest, ListVersionsRequest,
    get_newest_ocsf_version as get_newest_version_impl, list_ocsf_versions as list_versions_impl,
};

use rmcp::{
    ErrorData as McpError, ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::*,
    tool, tool_handler, tool_router,
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

    #[tool(description = "List all available OCSF schema versions")]
    async fn list_ocsf_versions(
        &self,
        Parameters(request): Parameters<ListVersionsRequest>,
    ) -> Result<CallToolResult, McpError> {
        match list_versions_impl(request).await {
            Ok(result) => Ok(CallToolResult::success(vec![Content::text(result)])),
            Err(e) => Err(McpError::internal_error(
                "list_versions_error",
                Some(serde_json::json!({"error": e.to_string()})),
            )),
        }
    }

    #[tool(description = "Get the newest stable OCSF schema version")]
    async fn get_newest_ocsf_version(
        &self,
        Parameters(request): Parameters<GetNewestVersionRequest>,
    ) -> Result<CallToolResult, McpError> {
        match get_newest_version_impl(request).await {
            Ok(result) => Ok(CallToolResult::success(vec![Content::text(result)])),
            Err(e) => Err(McpError::internal_error(
                "get_newest_version_error",
                Some(serde_json::json!({"error": e.to_string()})),
            )),
        }
    }

    #[tool(description = "Read OCSF documentation and mapping guides")]
    async fn read_ocsf_docs(
        &self,
        Parameters(request): Parameters<ReadOcsfDocsRequest>,
    ) -> Result<CallToolResult, McpError> {
        match read_docs_impl(request).await {
            Ok(result) => Ok(CallToolResult::success(vec![Content::text(result)])),
            Err(e) => Err(McpError::internal_error(
                "read_docs_error",
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
                 Supports multiple OCSF schema versions (1.0.0 through 1.7.0-dev). \
                 Tools: browse_ocsf_schema, generate_ocsf_event, validate_ocsf_event, \
                 generate_logging_code, map_custom_to_ocsf, list_event_examples, \
                 list_ocsf_versions, get_newest_ocsf_version, read_ocsf_docs."
                    .to_string(),
            ),
        }
    }
}
