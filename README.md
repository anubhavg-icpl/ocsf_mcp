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

## 📊 OCSF Schema Support

Supports **OCSF v1.7.0-dev** (latest) with comprehensive coverage:

### 🏷️ Categories (8 total)
1. **System Activity** (UID: 1) - OS and device-level events (14 classes)
2. **Findings** (UID: 2) - Security findings (8 classes)
3. **Identity & Access Management** (UID: 3) - Auth and account events (6 classes)
4. **Network Activity** (UID: 4) - Network connections (14 classes)
5. **Discovery** (UID: 5) - Asset discovery (26 classes)
6. **Application Activity** (UID: 6) - Application events (8 classes)
7. **Remediation** (UID: 7) - Security remediation (4 classes)
8. **Unmanned Systems** (UID: 8) - Drone and autonomous systems (2 classes)

### 📈 Schema Statistics
- **82+ Event Classes** - Comprehensive coverage of security events
- **Multiple Versions** - Support for OCSF 1.0.0 through 1.7.0-dev
- **Rich Metadata** - Detailed object definitions and type system

### 🎯 Popular Event Classes
- `authentication` (UID: 3002) - Login, logout, auth failures
- `process_activity` (UID: 1007) - Process lifecycle events
- `file_activity` (UID: 1001) - File operations and access
- `network_activity` (UID: 4001) - Network connections and traffic
- `http_activity` (UID: 4002) - HTTP requests and responses
- `dns_activity` (UID: 4003) - DNS queries and responses
- `security_finding` (UID: 2001) - Security scan results
- `vulnerability_finding` (UID: 2006) - Vulnerability assessments

## 💡 Use Cases

### For AI Coding Assistants
1. **🔍 Understanding OCSF**: "Show me all event classes in the IAM category"
2. **⚡ Implementing Logging**: "Generate Python code for logging authentication events"
3. **🔄 Migration**: "Help me map this Splunk log to OCSF format"
4. **✅ Validation**: "Check if my event JSON is valid OCSF"
5. **📚 Learning**: "Show me examples of failed login events"

### For Developers
- 🚀 Quickly bootstrap OCSF logging in new applications
- ✅ Validate existing security logs against OCSF standards
- 🔄 Migrate from proprietary formats to OCSF
- 📖 Learn OCSF schema through interactive exploration

## 🎯 Practical Examples

### Example 1: Implementing Authentication Logging

**Step 1: Explore the schema**
```bash
browse_ocsf_schema(category="iam", show_attributes=true)
```

**Step 2: Generate example events**
```bash
list_event_examples(event_class="authentication")
```

**Step 3: Generate logging code**
```bash
generate_logging_code(
    language="python",
    event_classes="authentication",
    include_helpers=true
)
```

**Result**: Complete Python module with OCSF authentication logging!

### Example 2: Migrating Existing Logs

**Your current log:**
```
2025-01-15 10:30:00 ERROR [auth] Failed login attempt for user 'admin' from 192.168.1.100
```

**Step 1: Map to OCSF**
```bash
map_custom_to_ocsf(
    sample_log="2025-01-15 10:30:00 ERROR [auth] Failed login attempt for user 'admin' from 192.168.1.100",
    suggested_class="authentication"
)
```

**Step 2: Generate OCSF event**
```bash
generate_ocsf_event(
    event_class="authentication",
    required_fields="activity_id, time, type_uid",
    optional_fields='{"user": {"name": "admin"}, "src_endpoint": {"ip": "192.168.1.100"}, "status": "failure"}'
)
```

**Result**: Standards-compliant OCSF authentication event!

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

## 🧪 Testing

### Run Tests
```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Test specific module
cargo test schema_tests

# Integration tests only
cargo test --test integration_tests
```

### Test Coverage
- ✅ Schema loading and validation
- ✅ Event generation and validation
- ✅ Tool parameter parsing
- ✅ Version management
- ✅ Multi-version compatibility

## 🚀 Development

### Project Structure
```
src/
├── main.rs              # MCP server entry point
├── lib.rs               # Library exports
├── ocsf/                # Core OCSF engine
│   ├── schema.rs        # Schema parser and registry
│   ├── event.rs         # Event models and builders
│   ├── categories.rs    # Category definitions
│   └── validation.rs    # Event validation
├── tools/               # MCP tool implementations
│   ├── mod.rs           # Tool router and handlers
│   ├── schema_browser.rs
│   ├── event_generator.rs
│   ├── validator.rs
│   ├── code_generator.rs
│   ├── mapper.rs
│   └── version_tools.rs
└── templates/           # Code generation templates
    ├── python.rs
    ├── javascript.rs
    └── rust.rs
```

### Adding New Languages

1. Create new template file in `src/templates/`
2. Implement generator function
3. Add to `code_generator.rs` match statement
4. Add tests

### Adding New Event Classes

1. Update schema JSON files in `data/ocsf-schema/`
2. Add examples in `event.rs`
3. Update integration tests

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Add tests for new functionality
4. Ensure all tests pass (`cargo test`)
5. Update documentation
6. Submit a pull request

Contributions welcome! Areas for improvement:

- Full OCSF v1.3.0 schema integration (all event classes)
- Additional language templates (Go, Java, C#)
- Advanced validation with JSONSchema
- OCSF profile support
- Custom extension support

## 📄 License

MIT License - see LICENSE file for details.

## 🔗 Related Projects

- [OCSF Schema](https://schema.ocsf.io/) - Official OCSF schema and documentation
- [MCP SDK](https://github.com/modelcontextprotocol/rust-sdk) - Rust MCP SDK
- [Claude Desktop](https://claude.ai/desktop) - AI assistant with MCP support
- [OCSF GitHub](https://github.com/ocsf/ocsf-schema) - OCSF schema repository

## 📚 Resources

- [OCSF Schema Browser](https://schema.ocsf.io/)
- [OCSF GitHub](https://github.com/ocsf/ocsf-schema)
- [OCSF v1.7.0-dev Release](https://github.com/ocsf/ocsf-schema/releases)
- [Model Context Protocol](https://modelcontextprotocol.io/)
- [Rust MCP SDK](https://github.com/modelcontextprotocol/rust-sdk)

## 🙏 Credits

Built with:
- [rmcp](https://github.com/modelcontextprotocol/rust-sdk) - Official Rust MCP SDK
- [tokio](https://tokio.rs/) - Async runtime
- **OCSF Schema v1.7.0-dev** - Latest OCSF schema (83 classes, 168 objects)



---

**Made with ❤️ for the cybersecurity community**

*Enabling standardized security logging across all applications through AI-assisted development.*
