use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ReadOcsfDocsRequest {
    #[schemars(
        description = "Documentation topic (e.g., 'getting-started', 'event-classes', 'mapping-guide')"
    )]
    pub topic: String,
}

/// Read OCSF documentation and mapping guides
pub async fn read_ocsf_docs(request: ReadOcsfDocsRequest) -> Result<String> {
    tracing::info!("read_ocsf_docs called: topic={}", request.topic);

    let topic_lower = request.topic.to_lowercase();

    let doc_content = match topic_lower.as_str() {
        "getting-started" | "quickstart" | "intro" => r#"# OCSF Getting Started Guide

## What is OCSF?

The Open Cybersecurity Schema Framework (OCSF) is an open-source project that provides a
standardized schema for security event data. It enables interoperability between security
tools and simplifies data analysis.

## Core Concepts

### Event Classes
OCSF organizes events into classes grouped by categories:
- **System Activity** (Category 1): OS and device-level events
- **Findings** (Category 2): Security findings and detections
- **IAM** (Category 3): Authentication and authorization
- **Network Activity** (Category 4): Network connections and traffic
- **Discovery** (Category 5): Asset and resource discovery
- **Application Activity** (Category 6): Application-specific events
- **Remediation** (Category 7): Security remediation activities

### Event Structure
Every OCSF event has:
- `metadata`: Version, product info, timestamps
- `class_uid`: Unique ID for the event class
- `category_uid`: Category the event belongs to
- Event-specific attributes

## Quick Start

1. List available versions: Use `list_ocsf_versions` tool
2. Browse schema: Use `browse_ocsf_schema` to explore event classes
3. Generate events: Use `generate_ocsf_event` to create valid events
4. Generate code: Use `generate_logging_code` for implementation templates

## Resources
- Official OCSF website: https://schema.ocsf.io/
- GitHub: https://github.com/ocsf/ocsf-schema
"#
        .to_string(),
        "event-classes" | "classes" => r#"# OCSF Event Classes Guide

## Categories and Common Event Classes

### System Activity (Category 1)
- **File Activity** (1001): File operations (create, delete, modify)
- **Kernel Activity** (1003): Kernel-level events
- **Process Activity** (1007): Process lifecycle events

### Findings (Category 2)
- **Security Finding** (2001): Security detections and vulnerabilities
- **Detection Finding** (2004): Security detections

### Identity & Access Management (Category 3)
- **Authentication** (3002): Login, logout, auth events
- **Authorization** (3003): Access control decisions
- **Account Change** (3001): Account modifications

### Network Activity (Category 4)
- **Network Activity** (4001): Network connections
- **HTTP Activity** (4002): HTTP requests/responses
- **DNS Activity** (4003): DNS queries

### Discovery (Category 5)
- **Device Inventory** (5001): Asset discovery
- **User Inventory** (5002): User enumeration

### Application Activity (Category 6)
- **Web Resources Activity** (6003): Web app events
- **File Hosting** (6006): File hosting activity

### Remediation (Category 7)
- **Network Remediation** (7004): Network-level remediation actions

## How to Use

Use the `browse_ocsf_schema` tool with:
- No parameters: List all categories
- `category`: List event classes in a category
- `event_class`: Get details about a specific class
"#
        .to_string(),
        "mapping-guide" | "mapping" | "how-to-map" => r#"# OCSF Mapping Guide

## Steps to Map Custom Logs to OCSF

### 1. Identify the Event Type
Determine what kind of activity your log represents:
- Is it about authentication? → Authentication (3002)
- Process execution? → Process Activity (1007)
- File operations? → File Activity (1001)
- Network connection? → Network Activity (4001)

### 2. Map Required Fields
Every OCSF event requires:
- `time`: Event timestamp (ISO 8601 format)
- `class_uid`: Event class ID
- `category_uid`: Category ID
- `metadata`: Version and product info
- Event-specific required fields

### 3. Map Optional Fields
Add relevant optional fields:
- `severity_id`: Event severity (1=Info, 2=Low, 3=Medium, 4=High, 5=Critical)
- `status`: Operation status
- `unmapped`: Store fields that don't map to OCSF

### 4. Use Nested Objects
OCSF uses nested objects for complex data:
- `user`: User information (name, uid, email)
- `device`: Device information (hostname, IP, OS)
- `file`: File information (name, path, hash)
- `process`: Process information (name, pid, command line)

## Example Mapping

Custom log:
```json
{
  "timestamp": "2024-01-15T10:30:00Z",
  "event_type": "login_success",
  "username": "john.doe",
  "source_ip": "192.168.1.100"
}
```

OCSF mapping:
```json
{
  "class_uid": 3002,
  "category_uid": 3,
  "time": "2024-01-15T10:30:00Z",
  "metadata": {
    "version": "1.7.0-dev",
    "product": {
      "name": "My Auth System"
    }
  },
  "activity_id": 1,
  "user": {
    "name": "john.doe"
  },
  "src_endpoint": {
    "ip": "192.168.1.100"
  }
}
```

## Tools to Help

- Use `map_custom_to_ocsf` for automated mapping suggestions
- Use `browse_ocsf_schema` to explore available attributes
- Use `validate_ocsf_event` to check your mapped events
"#
        .to_string(),
        "best-practices" | "best-practice" => r#"# OCSF Best Practices

## Schema Version Management
- Use `list_ocsf_versions` to see available versions
- Use `get_newest_ocsf_version` for latest stable version
- Specify version in tool calls for consistency
- Test mappings against multiple versions if needed

## Event Generation
- Always include required fields (use `browse_ocsf_schema` to check)
- Use proper timestamp format (ISO 8601)
- Set appropriate severity levels
- Include `unmapped` object for fields that don't fit

## Code Generation
- Generate implementation code with `generate_logging_code`
- Support Rust, Python, and JavaScript
- Use builders for complex events
- Include validation in your implementation

## Validation
- Validate events before sending to consumers
- Use `validate_ocsf_event` tool
- Check for missing required fields
- Verify data types match schema

## Common Patterns

### Activity IDs
Most event classes have activity_id:
- 1: Success/Allow
- 2: Failure/Deny
- 99: Unknown

### Severity Levels
- 1: Informational
- 2: Low
- 3: Medium
- 4: High
- 5: Critical
- 6: Fatal

### Status Codes
- "Success": Operation succeeded
- "Failure": Operation failed
- "Unknown": Status unknown

## Performance Tips
- Cache loaded schemas
- Reuse event builders
- Batch validate when possible
- Use minimal attributes for high-volume events
"#
        .to_string(),
        "versions" | "version-guide" => r#"# OCSF Version Guide

## Available Versions

Our server supports multiple OCSF schema versions:
- **1.0.0** through **1.6.0**: Stable releases
- **1.7.0-dev**: Latest development version (83 classes, 168 objects)

## Version Differences

Major additions by version:
- **1.3.0**: Added remediation and OSINT events
- **1.4.0**: Enhanced discovery events
- **1.5.0**: Additional network events
- **1.6.0**: Extended application activity
- **1.7.0-dev**: Latest with all enhancements

## Choosing a Version

### For Production
Use `get_newest_ocsf_version` to get latest stable version.
Stable versions exclude dev/alpha/beta/rc markers.

### For Development
Use 1.7.0-dev for latest features and event classes.

### For Compatibility
Specify version parameter in all tool calls:
- `browse_ocsf_schema` with `version: "1.6.0"`
- `generate_ocsf_event` with `version: "1.6.0"`

## Migration Between Versions

1. List available versions with `list_ocsf_versions`
2. Test your mappings with new version
3. Check for new required fields
4. Update event generation code
5. Re-validate all events

## Version-Specific Tools

All tools support optional `version` parameter:
- If not specified: defaults to 1.7.0-dev
- If specified: loads that version's schema
- Invalid versions: falls back to minimal schema
"#
        .to_string(),
        _ => {
            format!(
                r#"# OCSF Documentation

Unknown topic: '{}'

## Available Topics

Use `read_ocsf_docs` with one of these topics:

- **getting-started**: Introduction to OCSF and quick start guide
- **event-classes**: Overview of all OCSF event classes by category
- **mapping-guide**: How to map custom logs to OCSF format
- **best-practices**: OCSF implementation best practices
- **versions**: Guide to OCSF schema versions

## Other Resources

- Browse schema: Use `browse_ocsf_schema` tool
- List versions: Use `list_ocsf_versions` tool
- Generate events: Use `generate_ocsf_event` tool
- Validate events: Use `validate_ocsf_event` tool
- Generate code: Use `generate_logging_code` tool
- Map logs: Use `map_custom_to_ocsf` tool
- See examples: Use `list_event_examples` tool

For detailed schema exploration, use the browse and generate tools rather than reading docs.
"#,
                request.topic
            )
        }
    };

    Ok(doc_content)
}
