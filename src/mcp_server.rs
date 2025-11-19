use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

// MCP Tool definitions
#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct McpTool {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct McpResponse {
    pub result: Option<Value>,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct AddSubtitleParams {
    pub id: Option<String>,
    pub text: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub background_color: String,
    pub text_color: String,
    pub font_size: f64,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct UpdateSubtitleParams {
    pub id: String,
    pub text: Option<String>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub background_color: Option<String>,
    pub text_color: Option<String>,
    pub font_size: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct RemoveSubtitleParams {
    pub id: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ToggleInteractionParams {
    pub enabled: Option<bool>,
}

pub fn get_mcp_tools() -> Vec<McpTool> {
    vec![
        McpTool {
            name: "add_subtitle".to_string(),
            description: "Add a new subtitle to the overlay".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "id": {
                        "type": "string",
                        "description": "Unique identifier for the subtitle (optional, auto-generated if not provided)"
                    },
                    "text": {
                        "type": "string",
                        "description": "Text content of the subtitle"
                    },
                    "x": {
                        "type": "number",
                        "description": "X position in pixels"
                    },
                    "y": {
                        "type": "number", 
                        "description": "Y position in pixels"
                    },
                    "width": {
                        "type": "number",
                        "description": "Width in pixels"
                    },
                    "height": {
                        "type": "number",
                        "description": "Height in pixels"
                    },
                    "background_color": {
                        "type": "string",
                        "description": "Background color in hex format (#RRGGBB or #AARRGGBB)",
                        "default": "#CC000000"
                    },
                    "text_color": {
                        "type": "string",
                        "description": "Text color in hex format (#RRGGBB)",
                        "default": "#FFFFFF"
                    },
                    "font_size": {
                        "type": "number",
                        "description": "Font size in pixels",
                        "default": 16
                    }
                },
                "required": ["text", "x", "y", "width", "height"]
            }),
        },
        McpTool {
            name: "update_subtitle".to_string(),
            description: "Update an existing subtitle".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "id": {
                        "type": "string",
                        "description": "ID of the subtitle to update"
                    },
                    "text": {
                        "type": "string",
                        "description": "New text content (optional)"
                    },
                    "x": {
                        "type": "number",
                        "description": "New X position (optional)"
                    },
                    "y": {
                        "type": "number",
                        "description": "New Y position (optional)"
                    },
                    "width": {
                        "type": "number",
                        "description": "New width (optional)"
                    },
                    "height": {
                        "type": "number",
                        "description": "New height (optional)"
                    },
                    "background_color": {
                        "type": "string",
                        "description": "New background color (optional)"
                    },
                    "text_color": {
                        "type": "string",
                        "description": "New text color (optional)"
                    },
                    "font_size": {
                        "type": "number",
                        "description": "New font size (optional)"
                    }
                },
                "required": ["id"]
            }),
        },
        McpTool {
            name: "remove_subtitle".to_string(),
            description: "Remove a subtitle from the overlay".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "id": {
                        "type": "string",
                        "description": "ID of the subtitle to remove"
                    }
                },
                "required": ["id"]
            }),
        },
        McpTool {
            name: "clear_all_subtitles".to_string(),
            description: "Remove all subtitles from the overlay".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {},
                "required": []
            }),
        },
        McpTool {
            name: "list_subtitles".to_string(),
            description: "List all currently displayed subtitles".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {},
                "required": []
            }),
        },
        McpTool {
            name: "toggle_interaction".to_string(),
            description: "Enable or disable click-through interaction".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "enabled": {
                        "type": "boolean",
                        "description": "Enable (true) or disable (false) click-through. If not provided, toggles current state."
                    }
                },
                "required": []
            }),
        },
        McpTool {
            name: "set_always_on_top".to_string(),
            description: "Set whether the overlay window stays on top of other windows".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "enabled": {
                        "type": "boolean",
                        "description": "Enable (true) or disable (false) always-on-top"
                    }
                },
                "required": ["enabled"]
            }),
        },
        McpTool {
            name: "get_status".to_string(),
            description: "Get current status of the subtitle overlay".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {},
                "required": []
            }),
        },
    ]
}

// MCP response handlers
#[allow(dead_code)]
pub fn handle_add_subtitle(params: AddSubtitleParams) -> McpResponse {
    McpResponse {
        result: Some(json!({
            "success": true,
            "message": "Subtitle added successfully",
            "id": params.id.unwrap_or_else(|| "generated".to_string())
        })),
        error: None,
    }
}

#[allow(dead_code)]
pub fn handle_update_subtitle(params: UpdateSubtitleParams) -> McpResponse {
    McpResponse {
        result: Some(json!({
            "success": true,
            "message": "Subtitle updated successfully",
            "id": params.id
        })),
        error: None,
    }
}

#[allow(dead_code)]
pub fn handle_remove_subtitle(params: RemoveSubtitleParams) -> McpResponse {
    McpResponse {
        result: Some(json!({
            "success": true,
            "message": "Subtitle removed successfully",
            "id": params.id
        })),
        error: None,
    }
}

#[allow(dead_code)]
pub fn handle_clear_all() -> McpResponse {
    McpResponse {
        result: Some(json!({
            "success": true,
            "message": "All subtitles cleared"
        })),
        error: None,
    }
}

#[allow(dead_code)]
pub fn handle_list_subtitles() -> McpResponse {
    McpResponse {
        result: Some(json!({
            "success": true,
            "subtitles": [] // This would be populated from actual controller
        })),
        error: None,
    }
}

#[allow(dead_code)]
pub fn handle_toggle_interaction(params: ToggleInteractionParams) -> McpResponse {
    let enabled = params.enabled.unwrap_or_else(|| true); // Default to toggle
    McpResponse {
        result: Some(json!({
            "success": true,
            "message": if enabled { "Click-through enabled" } else { "Click-through disabled" },
            "click_through_enabled": enabled
        })),
        error: None,
    }
}

#[allow(dead_code)]
pub fn handle_set_always_on_top(enabled: bool) -> McpResponse {
    McpResponse {
        result: Some(json!({
            "success": true,
            "message": if enabled { "Always-on-top enabled" } else { "Always-on-top disabled" },
            "always_on_top": enabled
        })),
        error: None,
    }
}

#[allow(dead_code)]
pub fn handle_get_status() -> McpResponse {
    McpResponse {
        result: Some(json!({
            "success": true,
            "status": {
                "click_through_enabled": true,
                "always_on_top": true,
                "subtitle_count": 0
            }
        })),
        error: None,
    }
}

// MCP server initialization
#[allow(dead_code)]
pub fn initialize_mcp_server() -> Value {
    json!({
        "name": "subtitle-overlay",
        "version": "1.0.0",
        "description": "Subtitle Overlay API - Control on-screen subtitles programmatically",
        "tools": get_mcp_tools()
    })
}

// MCP protocol message handler
#[allow(dead_code)]
pub fn handle_mcp_request(method: &str, params: Value) -> McpResponse {
    match method {
        "tools/call" => {
            if let Some(tool_name) = params.get("name").and_then(|v| v.as_str()) {
                if let Some(args) = params.get("arguments") {
                    match tool_name {
                        "add_subtitle" => {
                            if let Ok(parsed) = serde_json::from_value::<AddSubtitleParams>(args.clone()) {
                                handle_add_subtitle(parsed)
                            } else {
                                McpResponse {
                                    result: None,
                                    error: Some("Invalid parameters for add_subtitle".to_string()),
                                }
                            }
                        }
                        "update_subtitle" => {
                            if let Ok(parsed) = serde_json::from_value::<UpdateSubtitleParams>(args.clone()) {
                                handle_update_subtitle(parsed)
                            } else {
                                McpResponse {
                                    result: None,
                                    error: Some("Invalid parameters for update_subtitle".to_string()),
                                }
                            }
                        }
                        "remove_subtitle" => {
                            if let Ok(parsed) = serde_json::from_value::<RemoveSubtitleParams>(args.clone()) {
                                handle_remove_subtitle(parsed)
                            } else {
                                McpResponse {
                                    result: None,
                                    error: Some("Invalid parameters for remove_subtitle".to_string()),
                                }
                            }
                        }
                        "clear_all_subtitles" => handle_clear_all(),
                        "list_subtitles" => handle_list_subtitles(),
                        "toggle_interaction" => {
                            if let Ok(parsed) = serde_json::from_value::<ToggleInteractionParams>(args.clone()) {
                                handle_toggle_interaction(parsed)
                            } else {
                                McpResponse {
                                    result: None,
                                    error: Some("Invalid parameters for toggle_interaction".to_string()),
                                }
                            }
                        }
                        "set_always_on_top" => {
                            if let Some(enabled) = args.get("enabled").and_then(|v| v.as_bool()) {
                                handle_set_always_on_top(enabled)
                            } else {
                                McpResponse {
                                    result: None,
                                    error: Some("Invalid parameters for set_always_on_top".to_string()),
                                }
                            }
                        }
                        "get_status" => handle_get_status(),
                        _ => McpResponse {
                            result: None,
                            error: Some(format!("Unknown tool: {}", tool_name)),
                        }
                    }
                } else {
                    McpResponse {
                        result: None,
                        error: Some("Missing arguments for tool call".to_string()),
                    }
                }
            } else {
                McpResponse {
                    result: None,
                    error: Some("Missing tool name".to_string()),
                }
            }
        }
        "tools/list" => {
            McpResponse {
                result: Some(json!(get_mcp_tools())),
                error: None,
            }
        }
        "initialize" => {
            McpResponse {
                result: Some(initialize_mcp_server()),
                error: None,
            }
        }
        _ => McpResponse {
            result: None,
            error: Some(format!("Unknown method: {}", method)),
        }
    }
}
