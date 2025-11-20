//! # Subs Overlay Library
//!
//! A library for creating transparent text overlays with input passthrough capabilities.
//!
//! This library allows you to:
//! - Create transparent text overlays on screen
//! - Make overlays ignore mouse/keyboard input (input passthrough)
//! - Keep overlays always on top
//! - Register and manage multiple overlay instances
//!
//! # Example
//!
//! ```rust
//! use subs_overlay_lib::{OverlayManager, OverlayConfig, TextConfig};
//!
//! // Create a new overlay manager
//! let manager = OverlayManager::new();
//!
//! // Configure the overlay text
//! let text_config = TextConfig {
//!     content: "Hello, World!".to_string(),
//!     font_size: 24.0,
//!     color: "#FFFFFFFF".to_string(), // White text
//!     position: (100, 100),
//! };
//!
//! // Configure the overlay
//! let overlay_config = OverlayConfig {
//!     text: text_config,
//!     width: 300,
//!     height: 100,
//!     transparent: true,
//!     always_on_top: true,
//!     ignore_input: true,
//! };
//!
//! // Create and show the overlay
//! let overlay_id = manager.create_overlay(overlay_config)?;
//! manager.show_overlay(&overlay_id)?;
//!
//! // Later, you can update or remove the overlay
//! manager.update_text(&overlay_id, "Updated text")?;
//! manager.remove_overlay(&overlay_id)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use once_cell::sync::Lazy;
use slint::{ComponentHandle, Weak};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
mod color_utils;
pub mod window_manager;

// Include the UI components
slint::include_modules!();

/// Type alias for overlay IDs
pub type OverlayId = String;

/// Configuration for text display in overlays
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TextConfig {
    /// Text content to display
    pub content: String,
    /// Font size in pixels
    pub font_size: f32,
    /// Text color in #AARRGGBB or #RRGGBB format
    pub color: String,
    /// Position (x, y) on screen
    pub position: (i32, i32),
}

/// Configuration for overlay windows
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OverlayConfig {
    /// Text configuration
    pub text: TextConfig,
    /// Window width in pixels
    pub width: i32,
    /// Window height in pixels
    pub height: i32,
    /// Whether the window should be transparent
    pub transparent: bool,
    /// Whether the window should always be on top
    pub always_on_top: bool,
    /// Whether the window should ignore input
    pub ignore_input: bool,
}

/// Manages multiple overlay instances
pub struct OverlayManager {
    overlays: Arc<Mutex<HashMap<OverlayId, OverlayWindow>>>,
}

struct OverlayWindow {
    window_weak: Weak<OverlayUI>,
    config: OverlayConfig,
}

// Thread-local storage to hold strong references to windows
// This is necessary because Slint windows are not Send and must be kept alive on the thread they were created.
thread_local! {
    static WINDOW_HOLDER: RefCell<HashMap<OverlayId, OverlayUI>> = RefCell::new(HashMap::new());
}

impl OverlayManager {
    /// Creates a new overlay manager
    pub fn new() -> Self {
        Self {
            overlays: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Creates a new overlay with the given configuration
    pub fn create_overlay(
        &self,
        config: OverlayConfig,
    ) -> Result<OverlayId, Box<dyn std::error::Error>> {
        let overlay_id = Uuid::new_v4().to_string();

        // Create the Slint window
        let ui = OverlayUI::new()?;

        // Set initial properties
        ui.set_text_content(config.text.content.clone().into());
        ui.set_font_size(config.text.font_size);

        // Convertir color hexadecimal a Slint Color
        let color_value = color_utils::hex_to_argb_u32(&config.text.color);

        ui.set_text_color(slint::Brush::from(slint::Color::from_argb_encoded(
            color_value,
        )));

        // Store the strong reference in thread-local storage to keep it alive
        WINDOW_HOLDER.with(|holder| {
            holder
                .borrow_mut()
                .insert(overlay_id.clone(), ui.clone_strong());
        });

        // Create overlay window structure with Weak reference
        let overlay_window = OverlayWindow {
            window_weak: ui.as_weak(),
            config: config.clone(),
        };

        // Store the overlay
        {
            let mut overlays = self.overlays.lock().unwrap();
            overlays.insert(overlay_id.clone(), overlay_window);
        }

        // Apply window properties (simplified for now)
        self.apply_window_properties(&overlay_id, &config)?;

        Ok(overlay_id)
    }

    /// Shows an overlay
    pub fn show_overlay(&self, overlay_id: &OverlayId) -> Result<(), Box<dyn std::error::Error>> {
        let overlays = self.overlays.lock().unwrap();

        if let Some(overlay) = overlays.get(overlay_id) {
            if let Some(window) = overlay.window_weak.upgrade() {
                // Establecer las propiedades de tamaño y color antes de mostrar
                window.set_win_width(overlay.config.width as f32);
                window.set_win_height(overlay.config.height as f32);
                // Removed incorrect text color override

                window.set_font_size(overlay.config.text.font_size);
                window.show()?;
            }
        }

        Ok(())
    }

    /// Hides an overlay
    pub fn hide_overlay(&self, overlay_id: &OverlayId) -> Result<(), Box<dyn std::error::Error>> {
        let overlays = self.overlays.lock().unwrap();

        if let Some(overlay) = overlays.get(overlay_id) {
            if let Some(window) = overlay.window_weak.upgrade() {
                window.hide()?;
            }
        }

        Ok(())
    }

    /// Updates the text of an overlay
    pub fn update_text(
        &self,
        overlay_id: &OverlayId,
        text: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut overlays = self.overlays.lock().unwrap();

        if let Some(overlay) = overlays.get_mut(overlay_id) {
            overlay.config.text.content = text.to_string();
            let text_content = text.to_string();

            self.execute_ui_action(&overlay.window_weak, move |window| {
                window.set_text_content(text_content.into());
            })?;
        }

        Ok(())
    }

    /// Updates the position of an overlay
    pub fn update_position(
        &self,
        overlay_id: &OverlayId,
        x: i32,
        y: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut overlays = self.overlays.lock().unwrap();

        if let Some(overlay) = overlays.get_mut(overlay_id) {
            overlay.config.text.position = (x, y);
        }

        Ok(())
    }

    /// Removes an overlay
    pub fn remove_overlay(&self, overlay_id: &OverlayId) -> Result<(), Box<dyn std::error::Error>> {
        let mut overlays = self.overlays.lock().unwrap();

        if let Some(_overlay) = overlays.remove(overlay_id) {
            // Remove from thread-local storage to drop the strong reference
            // We need to do this on the thread where it was created (or where the event loop is)
            // Since we don't know which thread we are on, we use invoke_from_event_loop
            let id_clone = overlay_id.clone();
            let _ = slint::invoke_from_event_loop(move || {
                WINDOW_HOLDER.with(|holder| {
                    holder.borrow_mut().remove(&id_clone);
                });
            });
        }

        Ok(())
    }

    /// Lists all active overlay IDs
    pub fn list_overlays(&self) -> Vec<OverlayId> {
        let overlays = self.overlays.lock().unwrap();
        overlays.keys().cloned().collect()
    }

    /// Gets the configuration of an overlay
    pub fn get_overlay_config(
        &self,
        overlay_id: &OverlayId,
    ) -> Result<OverlayConfig, Box<dyn std::error::Error>> {
        let overlays = self.overlays.lock().unwrap();

        if let Some(overlay) = overlays.get(overlay_id) {
            // Get the current text content
            let mut config = overlay.config.clone();
            if let Some(window) = overlay.window_weak.upgrade() {
                config.text.content = window.get_text_content().to_string();
            }
            Ok(config)
        } else {
            Err("Overlay not found".into())
        }
    }

    /// Applies window properties like transparency and input ignoring
    fn apply_window_properties(
        &self,
        overlay_id: &OverlayId,
        config: &OverlayConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut overlays = self.overlays.lock().unwrap();
        if let Some(overlay) = overlays.get_mut(overlay_id) {
            overlay.config = config.clone();

            let transparent = config.transparent;
            let always_on_top = config.always_on_top;

            self.execute_ui_action(&overlay.window_weak, move |window| {
                if let Ok(hwnd) = window_manager::get_native_handle(window.window()) {
                    if transparent {
                        let _ = window_manager::create_transparent_click_through_window(hwnd);
                    }
                    if always_on_top {
                        let _ = window_manager::set_always_on_top(hwnd, true);
                    }
                }
            })?;
        }

        Ok(())
    }

    /// Helper to execute actions on the UI thread
    fn execute_ui_action<F>(
        &self,
        window_weak: &Weak<OverlayUI>,
        action: F,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnOnce(OverlayUI) + Send + 'static,
    {
        let window_weak = window_weak.clone();
        slint::invoke_from_event_loop(move || {
            if let Some(window) = window_weak.upgrade() {
                action(window);
            }
        })?;
        Ok(())
    }
}

/// Global overlay manager instance
static GLOBAL_OVERLAY_MANAGER: Lazy<std::sync::Mutex<OverlayManager>> =
    Lazy::new(|| std::sync::Mutex::new(OverlayManager::new()));

/// Gets the global overlay manager instance
pub fn get_overlay_manager() -> &'static std::sync::Mutex<OverlayManager> {
    &GLOBAL_OVERLAY_MANAGER
}

/// Convenience function to create a simple text overlay
pub fn create_text_overlay(
    text: &str,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) -> Result<OverlayId, Box<dyn std::error::Error>> {
    let manager = get_overlay_manager().lock().unwrap();

    let text_config = TextConfig {
        content: text.to_string(),
        font_size: 24.0,
        color: "#FFFFFFFF".to_string(),
        position: (x, y),
    };

    let overlay_config = OverlayConfig {
        text: text_config,
        width,
        height,
        transparent: true,
        always_on_top: true,
        ignore_input: true,
    };

    let overlay_id = manager.create_overlay(overlay_config)?;
    manager.show_overlay(&overlay_id)?;

    Ok(overlay_id)
}

/// Convenience function to update an overlay's text
pub fn update_overlay_text(
    overlay_id: &OverlayId,
    text: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let manager = get_overlay_manager().lock().unwrap();

    // First try to update the text
    if let Err(e) = manager.update_text(overlay_id, text) {
        return Err(e);
    }

    // Then try to show the overlay (in case it's hidden)
    if let Err(e) = manager.show_overlay(overlay_id) {
        eprintln!("Warning: Could not show overlay after text update: {}", e);
    }

    Ok(())
}

/// Convenience function to remove an overlay
pub fn remove_overlay(overlay_id: &OverlayId) -> Result<(), Box<dyn std::error::Error>> {
    let manager = get_overlay_manager().lock().unwrap();
    manager.remove_overlay(overlay_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlay_creation() {
        let _manager = OverlayManager::new();

        let text_config = TextConfig {
            content: "Test".to_string(),
            font_size: 24.0,
            color: "#FFFFFFFF".to_string(),
            position: (100, 100),
        };

        let _overlay_config = OverlayConfig {
            text: text_config,
            width: 300,
            height: 100,
            transparent: true,
            always_on_top: true,
            ignore_input: true,
        };
    }

    #[test]
    fn test_overlay_persistence() {
        // This test verifies if the overlay window is kept alive after creation
        let manager = OverlayManager::new();
        let text_config = TextConfig {
            content: "Test Persistence".to_string(),
            font_size: 24.0,
            color: "#FFFFFFFF".to_string(),
            position: (100, 100),
        };
        let overlay_config = OverlayConfig {
            text: text_config,
            width: 300,
            height: 100,
            transparent: true,
            always_on_top: true,
            ignore_input: true,
        };

        if let Ok(overlay_id) = manager.create_overlay(overlay_config) {
            // Check if we can access the overlay
            let overlays = manager.overlays.lock().unwrap();
            if let Some(overlay) = overlays.get(&overlay_id) {
                // This is the critical check: can we upgrade the weak reference?
                // Since we are storing the strong reference in thread_local, this should succeed.
                assert!(
                    overlay.window_weak.upgrade().is_some(),
                    "Window should be alive"
                );
            } else {
                panic!("Overlay not found in manager");
            }
        } else {
            println!("Skipping test_overlay_persistence: Could not create overlay (no backend?)");
        }
    }
}
