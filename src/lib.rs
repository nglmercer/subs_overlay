use once_cell::sync::Lazy;
use slint::{ComponentHandle, Weak, EventLoopError, PlatformError};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use log;
mod color_utils;
pub mod window_manager;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OverlayError {
    #[error("Slint platform error: {source}")]
    SlintError {
        #[from]
        source: PlatformError,
    },
    #[error("Event loop error: {source}")]
    EventLoopError {
        #[from]
        source: EventLoopError,
    },
    #[error("Window manager error: {0}")]
    WindowManagerError(String),
    #[error("Overlay not found: {0}")]
    OverlayNotFound(String),
    #[error("Invalid color format: {0}")]
    InvalidColor(String),
    #[error("Lock acquisition failed")]
    LockError,
}

slint::include_modules!();

pub type OverlayId = String;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TextConfig {
    pub content: String,
    pub font_size: f32,
    pub color: String,
    pub position: (i32, i32),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OverlayConfig {
    pub text: TextConfig,
    pub width: i32,
    pub height: i32,
    pub transparent: bool,
    pub always_on_top: bool,
    pub ignore_input: bool,
}

pub struct OverlayManager {
    overlays: Arc<Mutex<HashMap<OverlayId, OverlayWindow>>>,
}

struct OverlayWindow {
    window_weak: Weak<OverlayUI>,
    config: OverlayConfig,
}

thread_local! {
    static WINDOW_HOLDER: RefCell<HashMap<OverlayId, OverlayUI>> = RefCell::new(HashMap::new());
}

impl OverlayManager {
    pub fn new() -> Self {
        Self {
            overlays: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn create_overlay(&self, config: OverlayConfig) -> Result<OverlayId, OverlayError> {
        if !color_utils::is_valid_color(&config.text.color) {
            return Err(OverlayError::InvalidColor(config.text.color.clone()));
        }

        let overlay_id = Uuid::new_v4().to_string();

        let ui = OverlayUI::new()?;

        ui.set_text_content(config.text.content.clone().into());
        ui.set_font_size(config.text.font_size);

        let color_value = color_utils::hex_to_argb_u32(&config.text.color);

        ui.set_text_color(slint::Brush::from(slint::Color::from_argb_encoded(color_value)));

        WINDOW_HOLDER.with(|holder| {
            holder.borrow_mut().insert(overlay_id.clone(), ui.clone_strong());
        });

        let overlay_window = OverlayWindow {
            window_weak: ui.as_weak(),
            config: config.clone(),
        };

        let mut overlays = self.overlays.lock().map_err(|_| OverlayError::LockError)?;
        overlays.insert(overlay_id.clone(), overlay_window);

        self.apply_window_properties(&overlay_id, &config)?;

        Ok(overlay_id)
    }

    pub fn show_overlay(&self, overlay_id: &OverlayId) -> Result<(), OverlayError> {
        let overlays = self.overlays.lock().map_err(|_| OverlayError::LockError)?;

        if let Some(overlay) = overlays.get(overlay_id) {
            if let Some(window) = overlay.window_weak.upgrade() {
                window.set_win_width(overlay.config.width as f32);
                window.set_win_height(overlay.config.height as f32);
                window.set_font_size(overlay.config.text.font_size);
                window.show()?;
            }
        }

        Ok(())
    }

    pub fn hide_overlay(&self, overlay_id: &OverlayId) -> Result<(), OverlayError> {
        let overlays = self.overlays.lock().map_err(|_| OverlayError::LockError)?;

        if let Some(overlay) = overlays.get(overlay_id) {
            if let Some(window) = overlay.window_weak.upgrade() {
                window.hide()?;
            }
        }

        Ok(())
    }

    pub fn update_text(&self, overlay_id: &OverlayId, text: &str) -> Result<(), OverlayError> {
        let mut overlays = self.overlays.lock().map_err(|_| OverlayError::LockError)?;

        if let Some(overlay) = overlays.get_mut(overlay_id) {
            overlay.config.text.content = text.to_string();
            let text_content = text.to_string();

            self.execute_ui_action(&overlay.window_weak, move |window| {
                window.set_text_content(text_content.into());
            })?;
        }

        Ok(())
    }

    pub fn update_position(&self, overlay_id: &OverlayId, x: i32, y: i32) -> Result<(), OverlayError> {
        let mut overlays = self.overlays.lock().map_err(|_| OverlayError::LockError)?;

        if let Some(overlay) = overlays.get_mut(overlay_id) {
            overlay.config.text.position = (x, y);
        }

        Ok(())
    }

    pub fn remove_overlay(&self, overlay_id: &OverlayId) -> Result<(), OverlayError> {
        let mut overlays = self.overlays.lock().map_err(|_| OverlayError::LockError)?;

        if overlays.remove(overlay_id).is_some() {
            let id_clone = overlay_id.clone();
            let _ = slint::invoke_from_event_loop(move || {
                WINDOW_HOLDER.with(|holder| {
                    holder.borrow_mut().remove(&id_clone);
                });
            });
        }

        Ok(())
    }

    pub fn list_overlays(&self) -> Vec<OverlayId> {
        self.overlays.lock().unwrap().keys().cloned().collect()
    }

    pub fn get_overlay_config(&self, overlay_id: &OverlayId) -> Result<OverlayConfig, OverlayError> {
        let overlays = self.overlays.lock().map_err(|_| OverlayError::LockError)?;

        if let Some(overlay) = overlays.get(overlay_id) {
            let mut config = overlay.config.clone();
            if let Some(window) = overlay.window_weak.upgrade() {
                config.text.content = window.get_text_content().to_string();
            }
            Ok(config)
        } else {
            Err(OverlayError::OverlayNotFound(overlay_id.clone()))
        }
    }

    fn apply_window_properties(&self, overlay_id: &OverlayId, config: &OverlayConfig) -> Result<(), OverlayError> {
        let mut overlays = self.overlays.lock().map_err(|_| OverlayError::LockError)?;
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

    fn execute_ui_action<F>(&self, window_weak: &Weak<OverlayUI>, action: F) -> Result<(), OverlayError>
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

static GLOBAL_OVERLAY_MANAGER: Lazy<Mutex<OverlayManager>> = Lazy::new(|| Mutex::new(OverlayManager::new()));

pub fn get_overlay_manager() -> &'static Mutex<OverlayManager> {
    &GLOBAL_OVERLAY_MANAGER
}

pub fn create_text_overlay(text: &str, x: i32, y: i32, width: i32, height: i32) -> Result<OverlayId, OverlayError> {
    let manager = get_overlay_manager().lock().map_err(|_| OverlayError::LockError)?;

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

pub fn update_overlay_text(overlay_id: &OverlayId, text: &str) -> Result<(), OverlayError> {
    let manager = get_overlay_manager().lock().map_err(|_| OverlayError::LockError)?;

    manager.update_text(overlay_id, text)?;

    if let Err(e) = manager.show_overlay(overlay_id) {
        log::warn!("Could not show overlay after text update: {}", e);
    }

    Ok(())
}

pub fn remove_overlay(overlay_id: &OverlayId) -> Result<(), OverlayError> {
    let manager = get_overlay_manager().lock().map_err(|_| OverlayError::LockError)?;
    manager.remove_overlay(overlay_id)
}
