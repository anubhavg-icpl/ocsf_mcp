# OCSF MCP Server Configuration

This directory contains the MCP (Model Context Protocol) configuration for the OCSF MCP Server.

## Configuration

The `mcp.json` file configures Claude Code to use the OCSF MCP server:

```json
{
  "mcpServers": {
    "ocsf": {
      "command": "/Users/anubhavg/Desktop/ocsf_mcp/target/release/ocsf-mcp-server",
      "args": [],
      "env": {},
      "disabled": false,
      "alwaysAllow": []
    }
  }
}
```

## Available Tools

The OCSF MCP server provides 9 tools:

### Version Management
1. **list_ocsf_versions** - List all available OCSF schema versions (1.0.0-1.7.0-dev)
2. **get_newest_ocsf_version** - Get the newest stable version

### Schema Operations
3. **browse_ocsf_schema** - Browse OCSF categories, event classes, and attributes
   - Supports version parameter
   - Filter by category or event class

### Event Operations
4. **generate_ocsf_event** - Generate valid OCSF events from parameters
5. **validate_ocsf_event** - Validate OCSF events against schema

### Code & Mapping
6. **generate_logging_code** - Generate OCSF logging code (Rust, Python, JavaScript)
7. **map_custom_to_ocsf** - Map custom log formats to OCSF
8. **list_event_examples** - Get example OCSF events for learning

### Documentation
9. **read_ocsf_docs** - Access embedded OCSF documentation
   - Topics: getting-started, event-classes, mapping-guide, best-practices, versions

## Usage

Once configured, you can use these tools in your Claude Code conversations:

```
Can you list all available OCSF versions?
→ Uses: list_ocsf_versions

Show me the authentication event class details
→ Uses: browse_ocsf_schema with category="iam" and event_class="authentication"

Generate an authentication event for a successful login
→ Uses: generate_ocsf_event with appropriate parameters

Generate Rust code for logging authentication events
→ Uses: generate_logging_code with language="rust"
```

## Server Status

To verify the server is working:

```bash
# Build the server
cargo build --release

# Test the server
./test_mcp.sh
```

The server should respond with a proper MCP initialize response listing all capabilities.

## Rebuilding

After making changes to the server code:

```bash
cd /Users/anubhavg/Desktop/ocsf_mcp
cargo build --release
```

Claude Code will automatically use the updated binary on the next tool invocation.

## Troubleshooting

If the MCP server isn't working:

1. Check the server binary exists:
   ```bash
   ls -lh target/release/ocsf-mcp-server
   ```

2. Test it manually:
   ```bash
   ./test_mcp.sh
   ```

3. Check Claude Code's MCP logs in the debug panel

4. Verify the path in `mcp.json` matches your actual binary location

## Architecture

See `ARCHITECTURE.md` in the project root for detailed information about the server architecture, data flows, and component interactions.
