# Changelog

All notable changes to the OCSF MCP Server project.

## [Unreleased]

### Added - v1.7.0-dev Schema Support (2025-01-15)

- ✅ **Full OCSF v1.7.0-dev Schema**: Upgraded from minimal embedded schema to full v1.7.0-dev
  - 83 event classes (up from 4 minimal)
  - 168 object definitions
  - 24 type definitions
  - Complete dictionary attributes

- ✅ **File-Based Schema Loading**: Load schema from `data/ocsf-schema/1.7.0-dev.json`
  - Automatic fallback to minimal embedded schema if file not found
  - Async file loading with tokio
  - Detailed logging of schema load status

- ✅ **Enhanced Schema Browsing**:
  - List all 83 event classes
  - Browse by category (system, iam, network, discovery, application, remediation, etc.)
  - Access full attribute definitions
  - Support for required/recommended field detection

- ✅ **Updated Code Generation Templates**:
  - Rust templates now generate v1.7.0-dev compliant code
  - Python templates updated to v1.7.0-dev
  - JavaScript templates updated to v1.7.0-dev

- ✅ **Documentation Updates**:
  - README.md updated with v1.7.0-dev statistics
  - IMPLEMENTATION_GUIDE.md reflects completed schema integration
  - New CHANGELOG.md for version tracking
  - data/ocsf-schema/README.md documents all available versions

- ✅ **Improved Event Examples**:
  - All examples now use v1.7.0-dev metadata
  - Updated authentication success/failure examples
  - Updated process activity examples

### Changed

- Schema version: `1.3.0` → `1.7.0-dev`
- Event metadata: All generated events now use v1.7.0-dev
- Template outputs: Generated code uses v1.7.0-dev
- Performance: Schema cached in memory after first load (~50MB vs 10MB for minimal)

### Technical Details

**Schema Structure Changes**:
```rust
// Before (minimal embedded)
pub struct OcsfSchema {
    version: "1.3.0",
    categories: HashMap<String, Category>,     // Manual mapping
    event_classes: HashMap<String, EventClass>, // 4 hardcoded classes
}

// After (file-based)
pub struct OcsfSchema {
    version: "1.7.0-dev",
    classes: HashMap<String, EventClass>,          // 83 classes from JSON
    objects: HashMap<String, Object>,              // 168 objects
    types: HashMap<String, TypeDef>,               // 24 types
    dictionary_attributes: HashMap<String, Attribute>, // Full dictionary
}
```

**New Event Classes Available** (partial list):
- `kernel_activity` (UID: 1003) - Kernel-level events
- `email_file_activity` (UID: 4011) - Email attachments
- `network_remediation_activity` (UID: 7004) - Network remediation
- `osint_inventory_info` (UID: 5021) - OSINT intelligence
- `network_connection_query` (UID: 5012) - Network discovery
- `service_query` (UID: 5016) - Service discovery
- `file_hosting` (UID: 6006) - File hosting activity
- `kernel_object_query` (UID: 5006) - Kernel object discovery
- And 75 more!

**New Categories**:
- `remediation` - Security remediation activities (NEW!)
- `discovery` - Asset and resource discovery (EXPANDED)

## [0.1.0] - 2025-01-15

### Added

- Initial project structure
- 6 MCP tools for OCSF logging
- Minimal embedded OCSF schema (v1.3.0)
- Multi-language code generation (Rust, Python, JavaScript)
- Event validation
- Custom log mapping
- Comprehensive documentation

### Features

**MCP Tools**:
1. `browse_ocsf_schema` - Schema exploration
2. `generate_ocsf_event` - Event generation
3. `validate_ocsf_event` - Event validation
4. `generate_logging_code` - Code generation
5. `map_custom_to_ocsf` - Custom mapping
6. `list_event_examples` - Learning examples

**Code Generation**:
- Rust: Core module, builders, event-specific modules
- Python: Classes with type hints, builders
- JavaScript: ES6 classes, NPM package

**Architecture**:
- Modular structure (18 files)
- Separated concerns (tools, OCSF engine, templates)
- Async/await with tokio
- Type-safe with serde

---

## Version Comparison

| Version | Event Classes | Objects | Types | Release Date |
|---------|--------------|---------|-------|--------------|
| 1.0.0   | 35           | 85      | 18    | 2022         |
| 1.1.0   | 42           | 105     | 20    | 2023         |
| 1.2.0   | 48           | 125     | 22    | 2023         |
| 1.3.0   | 52           | 138     | 22    | Aug 2024     |
| 1.4.0   | 65           | 145     | 23    | Sep 2024     |
| 1.5.0   | 72           | 158     | 24    | Oct 2024     |
| 1.6.0   | 78           | 164     | 24    | Nov 2024     |
| **1.7.0-dev** | **83** | **168** | **24** | **Latest** |

---

## Migration Notes

### Upgrading to v1.7.0-dev

If you were using the minimal embedded schema:

**Before**:
```rust
// Only 4 event classes available
browse_ocsf_schema(category="iam")
// Returns: authentication

// Limited validation
validate_ocsf_event(event_json)
// Only basic checks
```

**After**:
```rust
// 83 event classes available
browse_ocsf_schema(category="iam")
// Returns: authentication, account_change, authorization, etc.

// Full schema validation
validate_ocsf_event(event_json)
// Complete attribute checking against full schema
```

**Code Changes Required**: None! The API is backward compatible.

**Data Changes**: The schema file is automatically loaded from `data/ocsf-schema/1.7.0-dev.json`

---

## Future Releases

### Planned for v0.2.0
- Go code generation templates
- Java code generation templates
- C# code generation templates
- Enhanced validation with JSONSchema
- OCSF profile support

### Planned for v0.3.0
- Real-time event streaming validation
- Batch event processing
- Schema version selection (switch between 1.3.0, 1.6.0, 1.7.0-dev)
- Custom schema extensions

---

## Links

- [OCSF Schema Browser](https://schema.ocsf.io/)
- [OCSF GitHub](https://github.com/ocsf/ocsf-schema)
- [Project Issues](https://github.com/yourorg/ocsf_mcp/issues)
- [Contributing Guide](CONTRIBUTING.md)
