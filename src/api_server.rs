use std::sync::Arc;
use tokio::sync::{RwLock, Mutex};
use warp::{Rejection, Reply};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::controller::{SubtitleController, SubtitleConfig, SubtitleUpdate};

// API request/response types
#[derive(Debug, Deserialize)]
pub struct AddSubtitleRequest {
    pub id: Option<String>,
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub background_color: String,
    pub text_color: String,
    pub font_size: f32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSubtitleRequest {
    pub text: Option<String>,
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub background_color: Option<String>,
    pub text_color: Option<String>,
    pub font_size: Option<f32>,
}

#[derive(Debug, Serialize)]
pub struct SubtitleResponse {
    pub id: String,
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub background_color: String,
    pub text_color: String,
    pub font_size: f32,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct StatusResponse {
    pub click_through_enabled: bool,
    pub always_on_top: bool,
    pub subtitle_count: usize,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}

// Global state for API server
pub struct ApiState {
    pub controller: Arc<RwLock<SubtitleController>>,
    pub click_through_enabled: Arc<Mutex<bool>>,
}

impl ApiState {
    pub fn new(controller: SubtitleController) -> Self {
        Self {
            controller: Arc::new(RwLock::new(controller)),
            click_through_enabled: Arc::new(Mutex::new(true)),
        }
    }
}

// Convert SubtitleConfig to SubtitleResponse
impl From<SubtitleConfig> for SubtitleResponse {
    fn from(config: SubtitleConfig) -> Self {
        Self {
            id: config.id,
            text: config.text,
            x: config.x,
            y: config.y,
            width: config.width,
            height: config.height,
            background_color: config.background_color,
            text_color: config.text_color,
            font_size: config.font_size,
        }
    }
}

// API endpoints
pub async fn add_subtitle(
    request: AddSubtitleRequest,
    state: Arc<ApiState>,
) -> Result<impl Reply, Rejection> {
    let id = request.id.unwrap_or_else(|| Uuid::new_v4().to_string());
    let config = SubtitleConfig {
        id: id.clone(),
        text: request.text,
        x: request.x,
        y: request.y,
        width: request.width,
        height: request.height,
        background_color: request.background_color,
        text_color: request.text_color,
        font_size: request.font_size,
    };

    let mut controller = state.controller.write().await;
    controller.add_subtitle(config);
    
    let subtitles = controller.get_subtitles();
    if let Some(subtitle) = subtitles.get(&id) {
        let response: SubtitleResponse = SubtitleResponse {
            id: subtitle.id.to_string(),
            text: subtitle.text.to_string(),
            x: subtitle.x,
            y: subtitle.y,
            width: subtitle.width,
            height: subtitle.height,
            background_color: subtitle.background_color.to_string(),
            text_color: subtitle.text_color.to_string(),
            font_size: subtitle.font_size,
        };
        Ok(warp::reply::json(&ApiResponse::success(response)))
    } else {
        Ok(warp::reply::json(&ApiResponse::<SubtitleResponse>::error(
            "Failed to add subtitle".to_string(),
        )))
    }
}

pub async fn update_subtitle(
    id: String,
    request: UpdateSubtitleRequest,
    state: Arc<ApiState>,
) -> Result<impl Reply, Rejection> {
    let updates = SubtitleUpdate {
        text: request.text,
        x: request.x,
        y: request.y,
        width: request.width,
        height: request.height,
        background_color: request.background_color,
        text_color: request.text_color,
        font_size: request.font_size,
    };

    let mut controller = state.controller.write().await;
    controller.update_subtitle(&id, updates);
    
    let subtitles = controller.get_subtitles();
    if let Some(subtitle) = subtitles.get(&id) {
        let response: SubtitleResponse = SubtitleResponse {
            id: subtitle.id.to_string(),
            text: subtitle.text.to_string(),
            x: subtitle.x,
            y: subtitle.y,
            width: subtitle.width,
            height: subtitle.height,
            background_color: subtitle.background_color.to_string(),
            text_color: subtitle.text_color.to_string(),
            font_size: subtitle.font_size,
        };
        Ok(warp::reply::json(&ApiResponse::success(response)))
    } else {
        Ok(warp::reply::json(&ApiResponse::<SubtitleResponse>::error(
            "Subtitle not found".to_string(),
        )))
    }
}

pub async fn remove_subtitle(
    id: String,
    state: Arc<ApiState>,
) -> Result<impl Reply, Rejection> {
    let mut controller = state.controller.write().await;
    controller.remove_subtitle(&id);
    
    Ok(warp::reply::json(&ApiResponse::success(true)))
}

pub async fn list_subtitles(
    state: Arc<ApiState>,
) -> Result<impl Reply, Rejection> {
    let controller = state.controller.read().await;
    let subtitles: Vec<SubtitleResponse> = controller
        .get_subtitles()
        .values()
        .map(|subtitle| SubtitleResponse {
            id: subtitle.id.to_string(),
            text: subtitle.text.to_string(),
            x: subtitle.x,
            y: subtitle.y,
            width: subtitle.width,
            height: subtitle.height,
            background_color: subtitle.background_color.to_string(),
            text_color: subtitle.text_color.to_string(),
            font_size: subtitle.font_size,
        })
        .collect();

    Ok(warp::reply::json(&ApiResponse::success(subtitles)))
}

pub async fn clear_all_subtitles(
    state: Arc<ApiState>,
) -> Result<impl Reply, Rejection> {
    let mut controller = state.controller.write().await;
    controller.clear_all();
    
    Ok(warp::reply::json(&ApiResponse::success(true)))
}

pub async fn toggle_click_through(
    state: Arc<ApiState>,
) -> Result<impl Reply, Rejection> {
    let mut click_through = state.click_through_enabled.lock().await;
    *click_through = !*click_through;
    let enabled = *click_through;
    
    // Update window properties through controller
    let _controller = state.controller.read().await;
    #[cfg(target_os = "windows")]
    crate::set_click_through(enabled).ok();
    
    Ok(warp::reply::json(&ApiResponse::success(enabled)))
}

pub async fn get_status(
    state: Arc<ApiState>,
) -> Result<impl Reply, Rejection> {
    let controller = state.controller.read().await;
    let click_through = state.click_through_enabled.lock().await;
    
    let status = StatusResponse {
        click_through_enabled: *click_through,
        always_on_top: true, // This would come from controller if implemented
        subtitle_count: controller.get_subtitles().len(),
    };

    Ok(warp::reply::json(&ApiResponse::success(status)))
}

// CORS handling
pub fn with_cors() -> warp::cors::Builder {
    warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
}
