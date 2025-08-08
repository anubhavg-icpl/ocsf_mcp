# OCSF Schema Data

This directory contains OCSF schema JSON files for offline use.

## Schema Sources

1. **Official OCSF Schema**: https://schema.ocsf.io/
2. **GitHub Repository**: https://github.com/ocsf/ocsf-schema

## Usage

### Option 1: Download Full Schema
```bash
curl -o schema.json https://schema.ocsf.io/export/schema
```

### Option 2: Use Embedded Minimal Schema
The server includes a minimal embedded schema in `src/ocsf/schema.rs` for:
- authentication (3002)
- process_activity (1007)
- And more...

### Option 3: Load from Remote (Runtime)
Configure `src/ocsf/schema.rs::OcsfSchema::load()` to fetch from remote:
```rust
let client = reqwest::Client::new();
let schema = client
    .get("https://schema.ocsf.io/api/schema/1.3.0")
    .send()
    .await?
    .json::<OcsfSchema>()
    .await?;
```

## Schema Structure

```
schema.json
├── version: "1.3.0"
├── categories: {...}
├── event_classes: {...}
├── objects: {...}
└── attributes: {...}
```

## Extending the Schema

To add custom extensions or profiles:

1. Place custom schema files here
2. Update `schema.rs` to merge with base schema
3. Validate against OCSF specification

## Version

Current OCSF Version: **1.7.0-dev** (Latest Development Release)

Available versions in this directory:
- `1.0.0.json` - Initial release
- `1.1.0.json` - First update
- `1.2.0.json` - Expanded schema
- `1.3.0.json` - August 2024 release (software inventory, remediation, OSINT)
- `1.4.0.json` - Enhanced discovery events
- `1.5.0.json` - Additional network events
- `1.6.0.json` - Extended application activity
- `1.7.0-dev.json` - **Latest** (83 classes, 168 objects, 24 types)

Latest v1.7.0-dev includes:
- Programmatic credential object
- Enhanced kernel activity events
- Network remediation activities
- OSINT inventory information
- Email file activity events
- Service query events
- And much more!
