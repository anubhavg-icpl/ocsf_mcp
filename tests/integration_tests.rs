use ocsf_mcp::ocsf::OcsfSchema;
use ocsf_mcp::tools::*;

#[cfg(test)]
mod version_tests {
    use super::*;

    #[test]
    fn test_list_versions() {
        let versions = OcsfSchema::list_versions().unwrap();
        assert!(!versions.is_empty(), "Should have at least one version");

        // Check versions are sorted
        let mut sorted = versions.clone();
        sorted.sort();
        assert_eq!(versions, sorted, "Versions should be sorted");

        // Check all versions are valid format
        for version in &versions {
            assert!(!version.is_empty(), "Version string should not be empty");
            assert!(
                version.contains('.') || version.contains('-'),
                "Version should contain . or -"
            );
        }
    }

    #[test]
    fn test_get_newest_stable_version() {
        let newest = OcsfSchema::get_newest_stable_version().unwrap();
        assert!(!newest.is_empty(), "Newest version should not be empty");

        // Check it doesn't contain development markers
        let version_lower = newest.to_lowercase();
        assert!(
            !version_lower.contains("dev"),
            "Stable version should not contain 'dev'"
        );
        assert!(
            !version_lower.contains("alpha"),
            "Stable version should not contain 'alpha'"
        );
        assert!(
            !version_lower.contains("beta"),
            "Stable version should not contain 'beta'"
        );
        assert!(
            !version_lower.contains("rc"),
            "Stable version should not contain 'rc'"
        );
    }

    #[tokio::test]
    async fn test_list_ocsf_versions_tool() {
        let request = ListVersionsRequest {};
        let result = list_versions_impl(request).await;

        assert!(result.is_ok(), "list_ocsf_versions should succeed");
        let response_json = result.unwrap();
        let response: serde_json::Value = serde_json::from_str(&response_json).unwrap();

        assert!(response["versions"].is_array());
        assert!(response["count"].is_number());
        assert!(response["count"].as_u64().unwrap() > 0);
    }

    #[tokio::test]
    async fn test_get_newest_ocsf_version_tool() {
        let request = GetNewestVersionRequest {};
        let result = get_newest_version_impl(request).await;

        assert!(result.is_ok(), "get_newest_ocsf_version should succeed");
        let response_json = result.unwrap();
        let response: serde_json::Value = serde_json::from_str(&response_json).unwrap();

        assert!(response["version"].is_string());
        assert!(response["is_stable"].as_bool().unwrap());
    }
}

#[cfg(test)]
mod schema_tests {
    use super::*;

    #[tokio::test]
    async fn test_load_default_version() {
        let schema = OcsfSchema::load().await;
        assert!(schema.is_ok(), "Should load default schema");

        let schema = schema.unwrap();
        assert_eq!(schema.version, "1.7.0-dev");
        assert!(!schema.classes.is_empty());
    }

    #[tokio::test]
    async fn test_load_valid_version() {
        let schema = OcsfSchema::load_version("1.3.0").await;
        assert!(schema.is_ok(), "Should load v1.3.0 schema");

        let schema = schema.unwrap();
        assert!(schema.version.contains("1.3"));
        assert!(!schema.classes.is_empty());
    }

    #[tokio::test]
    async fn test_load_invalid_version() {
        let schema = OcsfSchema::load_version("99.99.99").await;
        // Should fallback to minimal schema, not error
        assert!(schema.is_ok(), "Should handle invalid version gracefully");
    }

    #[tokio::test]
    async fn test_browse_schema_no_filters() {
        let request = BrowseSchemaRequest {
            version: None,
            category: None,
            event_class: None,
            show_attributes: false,
        };

        let result = browse_schema_impl(request).await;
        assert!(result.is_ok());

        let response_json = result.unwrap();
        assert!(response_json.contains("categories"));
    }

    #[tokio::test]
    async fn test_browse_schema_with_version() {
        let request = BrowseSchemaRequest {
            version: Some("1.6.0".to_string()),
            category: None,
            event_class: None,
            show_attributes: false,
        };

        let result = browse_schema_impl(request).await;
        assert!(result.is_ok());

        let response_json = result.unwrap();
        assert!(response_json.contains("1.6.0"));
    }

    #[tokio::test]
    async fn test_browse_schema_with_category() {
        let request = BrowseSchemaRequest {
            version: None,
            category: Some("iam".to_string()),
            event_class: None,
            show_attributes: false,
        };

        let result = browse_schema_impl(request).await;
        assert!(result.is_ok());

        let response_json = result.unwrap();
        assert!(response_json.contains("iam") || response_json.contains("authentication"));
    }

    #[tokio::test]
    async fn test_browse_schema_with_event_class() {
        let request = BrowseSchemaRequest {
            version: None,
            category: None,
            event_class: Some("authentication".to_string()),
            show_attributes: true,
        };

        let result = browse_schema_impl(request).await;
        assert!(result.is_ok());

        let response_json = result.unwrap();
        assert!(response_json.contains("authentication"));
        assert!(response_json.contains("UID") || response_json.contains("uid"));
    }
}

#[cfg(test)]
mod event_tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_generate_event_with_version() {
        let request = GenerateEventRequest {
            version: Some("1.7.0-dev".to_string()),
            event_class: "authentication".to_string(),
            required_fields: json!({"user": {"name": "testuser"}}).to_string(),
            optional_fields: None,
        };

        let result = generate_event_impl(request).await;
        assert!(result.is_ok());

        let event_json = result.unwrap();
        assert!(event_json.contains("authentication"));
        assert!(event_json.contains("testuser"));
    }

    #[tokio::test]
    async fn test_generate_event_invalid_class() {
        let request = GenerateEventRequest {
            version: None,
            event_class: "nonexistent_event_class_12345".to_string(),
            required_fields: "{}".to_string(),
            optional_fields: None,
        };

        let result = generate_event_impl(request).await;
        assert!(result.is_err(), "Should fail for invalid event class");
    }
}

#[cfg(test)]
mod tool_integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_all_versions_loadable() {
        let versions = OcsfSchema::list_versions().unwrap();

        for version in versions {
            let schema = OcsfSchema::load_version(&version).await;
            assert!(schema.is_ok(), "Should be able to load version {version}");

            let schema = schema.unwrap();
            assert!(
                !schema.classes.is_empty(),
                "Schema {version} should have classes"
            );
        }
    }

    #[tokio::test]
    async fn test_version_compatibility_chain() {
        // Test that we can list versions, get newest, and load it
        let versions = OcsfSchema::list_versions().unwrap();
        assert!(!versions.is_empty());

        let newest = OcsfSchema::get_newest_stable_version().unwrap();
        assert!(versions.contains(&newest));

        let schema = OcsfSchema::load_version(&newest).await.unwrap();
        assert!(!schema.classes.is_empty());
    }
}
