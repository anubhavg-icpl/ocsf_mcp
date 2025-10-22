#!/bin/bash

# Comprehensive MCP Server Test Suite
# Tests all 9 OCSF MCP tools with various scenarios

SERVER_BIN="/Users/anubhavg/Desktop/ocsf_mcp/target/release/ocsf-mcp-server"
TEST_LOG="mcp_test_results.log"

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test counter
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Function to print test header
print_test() {
    echo ""
    echo "=================================="
    echo "TEST: $1"
    echo "=================================="
}

# Function to send MCP request and check response
test_tool() {
    local test_name="$1"
    local tool_name="$2"
    local params="$3"

    TOTAL_TESTS=$((TOTAL_TESTS + 1))

    echo -e "${YELLOW}Testing: $test_name${NC}"

    # Create the request
    local request=$(cat <<EOF
{"jsonrpc":"2.0","id":$TOTAL_TESTS,"method":"tools/call","params":{"name":"$tool_name","arguments":$params}}
EOF
)

    # Send request and capture response
    echo "$request" | timeout 5s "$SERVER_BIN" 2>&1 | head -20 > /tmp/mcp_response.txt

    # Check if we got a valid response
    if grep -q '"result"' /tmp/mcp_response.txt; then
        echo -e "${GREEN}✓ PASSED${NC}"
        PASSED_TESTS=$((PASSED_TESTS + 1))
        echo "Request: $request" >> "$TEST_LOG"
        echo "Response:" >> "$TEST_LOG"
        cat /tmp/mcp_response.txt >> "$TEST_LOG"
        echo "" >> "$TEST_LOG"

        # Show brief response
        grep '"result"' /tmp/mcp_response.txt | head -3
    elif grep -q '"error"' /tmp/mcp_response.txt; then
        echo -e "${RED}✗ FAILED (error returned)${NC}"
        FAILED_TESTS=$((FAILED_TESTS + 1))
        cat /tmp/mcp_response.txt
    else
        echo -e "${RED}✗ FAILED (no response)${NC}"
        FAILED_TESTS=$((FAILED_TESTS + 1))
    fi
}

# Initialize log
echo "OCSF MCP Server Comprehensive Test Suite" > "$TEST_LOG"
echo "Test run: $(date)" >> "$TEST_LOG"
echo "========================================" >> "$TEST_LOG"
echo "" >> "$TEST_LOG"

echo "======================================"
echo "OCSF MCP SERVER COMPREHENSIVE TEST SUITE"
echo "======================================"
echo ""
echo "Server: $SERVER_BIN"
echo "Log: $TEST_LOG"
echo ""

# ==========================================
# 1. VERSION MANAGEMENT TOOLS (2 tests)
# ==========================================
print_test "1. Version Management Tools"

test_tool \
    "list_ocsf_versions - List all available versions" \
    "list_ocsf_versions" \
    '{}'

test_tool \
    "get_newest_ocsf_version - Get newest stable version" \
    "get_newest_ocsf_version" \
    '{}'

# ==========================================
# 2. SCHEMA BROWSING TOOLS (4 tests)
# ==========================================
print_test "2. Schema Browsing Tools"

test_tool \
    "browse_ocsf_schema - List all categories" \
    "browse_ocsf_schema" \
    '{"version":null,"category":null,"event_class":null,"show_attributes":false}'

test_tool \
    "browse_ocsf_schema - List event classes in IAM category" \
    "browse_ocsf_schema" \
    '{"version":"1.7.0-dev","category":"iam","event_class":null,"show_attributes":false}'

test_tool \
    "browse_ocsf_schema - Get authentication event class details" \
    "browse_ocsf_schema" \
    '{"version":"1.7.0-dev","category":null,"event_class":"authentication","show_attributes":false}'

test_tool \
    "browse_ocsf_schema - Get authentication with attributes" \
    "browse_ocsf_schema" \
    '{"version":"1.7.0-dev","category":null,"event_class":"authentication","show_attributes":true}'

# ==========================================
# 3. EVENT GENERATION TOOLS (3 tests)
# ==========================================
print_test "3. Event Generation Tools"

test_tool \
    "generate_ocsf_event - Generate authentication event" \
    "generate_ocsf_event" \
    '{"version":"1.7.0-dev","event_class":"authentication","required_fields":"{\"user\":{\"name\":\"testuser\"},\"activity_id\":1}","optional_fields":null}'

test_tool \
    "generate_ocsf_event - Generate file activity event" \
    "generate_ocsf_event" \
    '{"version":"1.6.0","event_class":"file","required_fields":"{\"file\":{\"name\":\"test.txt\",\"path\":\"/tmp/test.txt\"}}","optional_fields":"{\"severity_id\":1}"}'

test_tool \
    "generate_ocsf_event - Generate with optional fields" \
    "generate_ocsf_event" \
    '{"version":"1.7.0-dev","event_class":"authentication","required_fields":"{\"user\":{\"name\":\"john.doe\"}}","optional_fields":"{\"severity_id\":2,\"status\":\"Success\"}"}'

# ==========================================
# 4. EVENT VALIDATION TOOLS (3 tests)
# ==========================================
print_test "4. Event Validation Tools"

# First generate a valid event to validate
VALID_EVENT='{"class_uid":3002,"category_uid":3,"time":"2024-10-24T00:00:00Z","metadata":{"version":"1.7.0-dev","product":{"name":"Test"}},"user":{"name":"testuser"}}'

test_tool \
    "validate_ocsf_event - Validate valid event" \
    "validate_ocsf_event" \
    "{\"version\":\"1.7.0-dev\",\"event_json\":\"$VALID_EVENT\"}"

INVALID_EVENT='{"class_uid":9999,"category_uid":99}'

test_tool \
    "validate_ocsf_event - Validate invalid event (bad class_uid)" \
    "validate_ocsf_event" \
    "{\"version\":\"1.7.0-dev\",\"event_json\":\"$INVALID_EVENT\"}"

MISSING_FIELDS='{"class_uid":3002,"category_uid":3}'

test_tool \
    "validate_ocsf_event - Validate event with missing fields" \
    "validate_ocsf_event" \
    "{\"version\":\"1.7.0-dev\",\"event_json\":\"$MISSING_FIELDS\"}"

# ==========================================
# 5. CODE GENERATION TOOLS (3 tests)
# ==========================================
print_test "5. Code Generation Tools"

test_tool \
    "generate_logging_code - Generate Rust code" \
    "generate_logging_code" \
    '{"language":"rust","event_classes":"authentication","framework":null,"include_helpers":true}'

test_tool \
    "generate_logging_code - Generate Python code" \
    "generate_logging_code" \
    '{"language":"python","event_classes":"authentication,file","framework":null,"include_helpers":true}'

test_tool \
    "generate_logging_code - Generate JavaScript code" \
    "generate_logging_code" \
    '{"language":"javascript","event_classes":"authentication","framework":"express","include_helpers":false}'

# ==========================================
# 6. MAPPING TOOLS (2 tests)
# ==========================================
print_test "6. Mapping Tools"

test_tool \
    "map_custom_to_ocsf - Map custom authentication log" \
    "map_custom_to_ocsf" \
    '{"custom_log":"{\"event\":\"login\",\"user\":\"john\",\"result\":\"success\"}","hints":"authentication event"}'

test_tool \
    "list_event_examples - List authentication examples" \
    "list_event_examples" \
    '{"category":"iam","limit":2}'

# ==========================================
# 7. DOCUMENTATION TOOLS (5 tests)
# ==========================================
print_test "7. Documentation Tools"

test_tool \
    "read_ocsf_docs - Get getting-started guide" \
    "read_ocsf_docs" \
    '{"topic":"getting-started"}'

test_tool \
    "read_ocsf_docs - Get event-classes documentation" \
    "read_ocsf_docs" \
    '{"topic":"event-classes"}'

test_tool \
    "read_ocsf_docs - Get mapping-guide" \
    "read_ocsf_docs" \
    '{"topic":"mapping-guide"}'

test_tool \
    "read_ocsf_docs - Get best-practices" \
    "read_ocsf_docs" \
    '{"topic":"best-practices"}'

test_tool \
    "read_ocsf_docs - Get versions guide" \
    "read_ocsf_docs" \
    '{"topic":"versions"}'

# ==========================================
# 8. EDGE CASES (5 tests)
# ==========================================
print_test "8. Edge Cases and Error Handling"

test_tool \
    "browse_ocsf_schema - Invalid version" \
    "browse_ocsf_schema" \
    '{"version":"99.99.99","category":null,"event_class":null,"show_attributes":false}'

test_tool \
    "browse_ocsf_schema - Non-existent category" \
    "browse_ocsf_schema" \
    '{"version":"1.7.0-dev","category":"nonexistent","event_class":null,"show_attributes":false}'

test_tool \
    "generate_ocsf_event - Invalid event class" \
    "generate_ocsf_event" \
    '{"version":"1.7.0-dev","event_class":"invalid_event_class_12345","required_fields":"{}","optional_fields":null}'

test_tool \
    "generate_ocsf_event - Malformed JSON in required_fields" \
    "generate_ocsf_event" \
    '{"version":"1.7.0-dev","event_class":"authentication","required_fields":"{invalid json","optional_fields":null}'

test_tool \
    "read_ocsf_docs - Unknown topic" \
    "read_ocsf_docs" \
    '{"topic":"nonexistent-topic"}'

# ==========================================
# SUMMARY
# ==========================================
echo ""
echo "======================================"
echo "TEST SUMMARY"
echo "======================================"
echo "Total Tests: $TOTAL_TESTS"
echo -e "Passed: ${GREEN}$PASSED_TESTS${NC}"
echo -e "Failed: ${RED}$FAILED_TESTS${NC}"
echo ""

if [ $FAILED_TESTS -eq 0 ]; then
    echo -e "${GREEN}All tests passed! ✓${NC}"
    exit 0
else
    echo -e "${RED}Some tests failed. Check $TEST_LOG for details.${NC}"
    exit 1
fi
