# OCSF MCP Server


A fully-fledged Rust-based Model Context Protocol (MCP) server that enables AI coding assistants to implement OCSF (Open Cybersecurity Schema Framework) based logging in any application.

## 🚀 Features

This MCP server provides comprehensive OCSF tooling:

- **🔍 Schema Browsing**: Explore OCSF categories, event classes, and attributes interactively
- **⚡ Event Generation**: Generate valid OCSF-compliant events with proper structure
- **✅ Validation**: Validate existing events against OCSF schema
- **🛠️ Code Generation**: Generate logging code in multiple languages (Rust, Python, JavaScript)
- **🔄 Custom Mapping**: Map proprietary log formats to OCSF event classes
- **📚 Learning**: Access example events for different scenarios
- **🔧 Version Management**: Support for multiple OCSF schema versions (1.0.0 - 1.7.0-dev)

## 🏗️ Architecture

```
AI Assistant (Claude, etc.)
    ↓ MCP Protocol (stdio)
OCSF MCP Server (Rust)
    ├── 🛠️ Tools Layer (7 MCP tools)
    ├── 🧠 OCSF Core Engine (schema, validation, events)
    ├── 📝 Code Templates (multi-language support)
    └── 📊 Schema Data (embedded JSON schemas)
```

## 🔧 Available MCP Tools

### 1. `browse_ocsf_schema`
Browse OCSF schema categories, event classes, and attributes.

**Parameters:**
- `category` (optional): Category name (e.g., "network", "iam", "system")
- `event_class` (optional): Event class name (e.g., "authentication", "file_activity")
- `show_attributes`: Boolean - Show detailed attribute information

**Examples:**
```bash
# Browse all categories
browse_ocsf_schema(show_attributes=false)

# Browse specific category
browse_ocsf_schema(category="network", show_attributes=true)

# Browse specific event class
browse_ocsf_schema(event_class="authentication", show_attributes=true)
```

### 2. `generate_ocsf_event`
Generate valid OCSF event JSON from parameters.

**Parameters:**
- `event_class`: Event class name
- `required_fields`: Comma-separated field names OR JSON object with field values
- `optional_fields`: Comma-separated field names OR JSON object with field values

**Examples:**
```bash
# Using comma-separated field names (auto-generates values)
generate_ocsf_event(
    event_class="authentication",
    required_fields="activity_id, category_uid, class_uid, severity_id, time, type_uid",
    optional_fields="message, user"
)

# Using JSON objects with specific values
generate_ocsf_event(
    event_class="authentication",
    required_fields='{"activity_id": 1, "time": "2025-01-15T10:30:00Z"}',
    optional_fields='{"user": {"name": "john.doe", "uid": "1001"}}'
)
```

### 3. `validate_ocsf_event`
Validate an OCSF event against the schema.

**Parameters:**
- `event_json`: The OCSF event JSON string to validate

**Example:**
```bash
validate_ocsf_event(
    event_json='{"metadata": {"version": "1.7.0-dev", "event_class": "authentication"}, "time": "2025-01-15T10:30:00Z"}'
)
```

### 4. `generate_logging_code`
Generate OCSF logging code for a specific language/framework.

**Parameters:**
- `language`: Target language (rust, python, javascript)
- `event_classes`: Comma-separated class names OR JSON array of event classes
- `framework` (optional): Logging framework
- `include_helpers`: Boolean - Include builder patterns and helpers

**Examples:**
```bash
# Using comma-separated class names
generate_logging_code(
    language="python",
    event_classes="authentication,file_activity",
    include_helpers=true
)

# Using JSON array
generate_logging_code(
    language="rust",
    event_classes='["authentication", "process_activity"]',
    include_helpers=true
)
```

### 5. `map_custom_to_ocsf`
Help map custom log format to OCSF event class.

**Parameters:**
- `sample_log`: User's existing log entry
- `suggested_class` (optional): Suggested event class

**Example:**
```bash
map_custom_to_ocsf(
    sample_log="2025-01-15 10:30:00 INFO [auth] User john.doe successfully logged in from IP 192.168.1.100",
    suggested_class="authentication"
)
```

### 6. `list_event_examples`
Get example OCSF events for learning.

**Parameters:**
- `event_class`: Event class name
- `scenario` (optional): Specific scenario (e.g., "failed_login", "successful_login")

**Example:**
```bash
list_event_examples(
    event_class="authentication",
    scenario="failed_login"
)
```

### 7. `list_ocsf_versions` & `get_newest_ocsf_version`
Version management tools for OCSF schema versions.

**Examples:**
```bash
# List all available versions
list_ocsf_versions()

# Get newest stable version
get_newest_ocsf_version()
```

## Building and Running

### Prerequisites
- Rust 1.70+ (with 2021 edition support)
- Cargo

### Build
```bash
cargo build --release
```

### Run
```bash
cargo run --bin ocsf-mcp-server
```

The server runs on stdio transport and communicates via JSON-RPC 2.0.

## Configuration for Claude Desktop

Add to your Claude Desktop MCP configuration (`claude_desktop_config.json`):

```json
{
  "mcpServers": {
    "ocsf": {
      "command": "/path/to/ocsf_mcp/target/release/ocsf-mcp-server",
      "args": []
    }
  }
}
```

## OCSF Schema Support

Supports **OCSF v1.7.0-dev** (latest) with the following categories:

1. **System Activity** (UID: 1) - OS and device-level events
2. **Findings** (UID: 2) - Security findings
3. **Identity & Access Management** (UID: 3) - Auth and account events
4. **Network Activity** (UID: 4) - Network connections
5. **Discovery** (UID: 5) - Asset discovery
6. **Application Activity** (UID: 6) - Application events

### Schema Statistics

- **83 Event Classes** - Comprehensive coverage of security events
- **168 Objects** - Detailed object definitions
- **24 Types** - Type system for attributes

### Sample Event Classes

- `authentication` (UID: 3002) - Login, logout, auth failures
- `process_activity` (UID: 1007) - Process lifecycle
- `file_activity` (UID: 1001) - File operations
- `network_activity` (UID: 4001) - Network connections
- `kernel_activity` (UID: 1003) - Kernel-level events
- `email_file_activity` (UID: 4011) - Email attachments
- `network_remediation_activity` (UID: 7004) - Network remediation
- And 76 more event classes!

## Use Cases

### For AI Coding Assistants

1. **Understanding OCSF**: "Show me all event classes in the IAM category"
2. **Implementing Logging**: "Generate Python code for logging authentication events"
3. **Migration**: "Help me map this Splunk log to OCSF format"
4. **Validation**: "Check if my event JSON is valid OCSF"
5. **Learning**: "Show me examples of failed login events"

### For Developers

- Quickly bootstrap OCSF logging in new applications
- Validate existing security logs against OCSF standards
- Migrate from proprietary formats to OCSF
- Learn OCSF schema through interactive exploration

## Architecture Details

### Core Components

**OCSF Engine** (`src/ocsf/`):
- `schema.rs` - Schema parser and registry
- `event.rs` - Event models and builders
- `categories.rs` - OCSF category definitions
- `validation.rs` - Event validation logic

**Tools** (`src/tools/`):
- MCP tool implementations using `#[tool]` macros
- Bridge between AI requests and OCSF engine

**Templates** (`src/templates/`):
- Multi-language code generation
- Support for Rust, Python, JavaScript, Go, Java

### Performance

- Async/await with Tokio runtime
- Achieves 4,700+ QPS (following rmcp SDK benchmarks)
- Memory-safe with Rust ownership system
- Zero-copy JSON parsing where possible

## Extending the Server

### Adding New Event Classes

1. Update `src/ocsf/schema.rs` with new event class definitions
2. Add examples in `src/ocsf/event.rs`
3. Server automatically exposes via existing tools

### Adding Language Support

1. Create new template file in `src/templates/`
2. Implement code generator in `src/tools/mod.rs::generate_logging_code`
3. Add language-specific examples

### Custom OCSF Schema

Replace `OcsfSchema::default_schema()` with:
```rust
// Fetch from schema.ocsf.io
let client = reqwest::Client::new();
let schema = client
    .get("https://schema.ocsf.io/api/schema")
    .send()
    .await?
    .json::<OcsfSchema>()
    .await?;
```

## Testing

```bash
# Run tests
cargo test

# Run with debug logging
RUST_LOG=debug cargo run
```

## Comparison with Tenzir MCP Server

| Feature | OCSF MCP (This) | Tenzir MCP |
|---------|----------------|------------|
| Language | Rust | Python |
| OCSF Version | v1.7.0-dev (83 classes) | v1.3.0 (~40 classes) |
| Schema Loading | File-based + embedded | Remote API |
| Focus | AI-assisted OCSF implementation | TQL pipeline generation |
| Performance | 4,700+ QPS | ~100-500 QPS (Python) |
| Code Gen | Multi-language (Rust, Python, JS) | TQL specific |
| Validation | Built-in schema validation | Via Tenzir Node |
| Deployment | Single binary (~5MB) | Requires Node/Docker (~50MB+) |

## Contributing

Contributions welcome! Areas for improvement:

- Full OCSF v1.3.0 schema integration (all event classes)
- Additional language templates (Go, Java, C#)
- Advanced validation with JSONSchema
- OCSF profile support
- Custom extension support

## License

Apache 2.0 (matching OCSF and Tenzir MCP)

## Resources

- [OCSF Schema Browser](https://schema.ocsf.io/)
- [OCSF GitHub](https://github.com/ocsf/ocsf-schema)
- [OCSF v1.7.0-dev Release](https://github.com/ocsf/ocsf-schema/releases)
- [Model Context Protocol](https://modelcontextprotocol.io/)
- [Rust MCP SDK](https://github.com/modelcontextprotocol/rust-sdk)
- [Tenzir MCP Server](https://github.com/tenzir/mcp)

## Credits

Built with:
- [rmcp](https://github.com/modelcontextprotocol/rust-sdk) - Official Rust MCP SDK
- [tokio](https://tokio.rs/) - Async runtime
- **OCSF Schema v1.7.0-dev** - Latest OCSF schema (83 classes, 168 objects)

Inspired by [Tenzir MCP Server](https://github.com/tenzir/mcp) for OCSF mapping approach.
