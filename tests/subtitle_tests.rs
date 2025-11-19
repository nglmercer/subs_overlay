#[cfg(test)]
mod tests {
    use slint_subtitles::controller::{SubtitleController, SubtitleConfig};
    use slint_subtitles::color_utils::{to_slint_color_string, is_valid_color};

    #[test]
    fn test_subtitle_creation() {
        // This test would require creating a mock UI or using headless mode
        // For now, we'll test the configuration creation logic
        let config = SubtitleConfig {
            id: "test-sub".to_string(),
            text: "Test subtitle".to_string(),
            x: 100.0,
            y: 200.0,
            width: 300.0,
            height: 50.0,
            background_color: "#CC000000".to_string(),
            text_color: "#FFFFFF".to_string(),
            font_size: 16.0,
        };

        assert_eq!(config.id, "test-sub");
        assert_eq!(config.text, "Test subtitle");
        assert_eq!(config.x, 100.0);
        assert_eq!(config.y, 200.0);
        assert_eq!(config.width, 300.0);
        assert_eq!(config.height, 50.0);
        assert_eq!(config.background_color, "#CC000000");
        assert_eq!(config.text_color, "#FFFFFF");
        assert_eq!(config.font_size, 16.0);
    }

    #[test]
    fn test_color_utilities() {
        // Test color conversion
        assert_eq!(to_slint_color_string("#CC000000"), "#000000");
        assert_eq!(to_slint_color_string("#FFFFFFFF"), "#FFFFFF");
        assert_eq!(to_slint_color_string("#FF0000"), "#FF0000");
        
        // Test color validation
        assert!(is_valid_color("#FF0000"));
        assert!(is_valid_color("#CCFF0000"));
        assert!(!is_valid_color("FF0000")); // Missing #
        assert!(!is_valid_color("#FF00"));  // Too short
        assert!(!is_valid_color("#FF000000")); // Too long
    }

    #[test]
    fn test_mcp_functionality() {
        // Test MCP tool definitions
        let tools = slint_subtitles::mcp_server::get_mcp_tools();
        assert!(!tools.is_empty());
        
        // Check that expected tools are present
        let tool_names: Vec<String> = tools.iter().map(|t| t.name.clone()).collect();
        assert!(tool_names.contains(&"add_subtitle".to_string()));
        assert!(tool_names.contains(&"update_subtitle".to_string()));
        assert!(tool_names.contains(&"remove_subtitle".to_string()));
        assert!(tool_names.contains(&"list_subtitles".to_string()));
        assert!(tool_names.contains(&"clear_all_subtitles".to_string()));
        assert!(tool_names.contains(&"toggle_interaction".to_string()));
        assert!(tool_names.contains(&"set_always_on_top".to_string()));
        assert!(tool_names.contains(&"get_status".to_string()));
    }

    #[test]
    fn test_mcp_request_handling() {
        use slint_subtitles::mcp_server::{AddSubtitleParams, handle_add_subtitle};
        use serde_json::json;

        // Test add subtitle request
        let params = AddSubtitleParams {
            id: Some("test".to_string()),
            text: "Test subtitle".to_string(),
            x: 100.0,
            y: 200.0,
            width: 300.0,
            height: 50.0,
            background_color: "#CC000000".to_string(),
            text_color: "#FFFFFF".to_string(),
            font_size: 16.0,
        };

        let response = handle_add_subtitle(params);
        assert!(response.result.is_some());
        assert!(response.error.is_none());
        
        if let Some(result) = response.result {
            assert_eq!(result["success"], true);
            assert_eq!(result["message"], "Subtitle added successfully");
            assert_eq!(result["id"], "test");
        }
    }

    #[test]
    fn test_config_default_values() {
        use slint_subtitles::config::AppConfig;

        let config = AppConfig::default();
        assert_eq!(config.server.host, "127.0.0.1");
        assert_eq!(config.server.port, 8080);
        assert!(config.server.cors_enabled);
        assert!(config.window.always_on_top);
        assert!(config.window.click_through);
        assert!(config.window.transparent);
        assert_eq!(config.window.default_width, 800);
        assert_eq!(config.window.default_height, 600);
        assert!(config.api.enabled);
        assert_eq!(config.api.rate_limit, 100);
        assert!(!config.api.auth_required);
        assert!(config.api.api_key.is_none());
        assert!(config.mcp.enabled);
        assert_eq!(config.mcp.log_level, "info");
        assert!(!config.mcp.tools_enabled.is_empty());
    }
}
