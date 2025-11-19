use std::collections::HashMap;
use std::rc::Rc;
use slint::Weak;

// Importar los tipos generados por Slint
use crate::{SubtitleWindow, SubtitleData};

/// Configuración para crear/actualizar un subtítulo
#[derive(Clone, Debug)]
pub struct SubtitleConfig {
    pub id: String,
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub background_color: String,  // Formato: #AARRGGBB o #RRGGBB
    pub text_color: String,
    pub font_size: f32,
}

/// Estructura para actualizaciones parciales
#[derive(Default)]
pub struct SubtitleUpdate {
    pub text: Option<String>,
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub background_color: Option<String>,
    pub text_color: Option<String>,
    pub font_size: Option<f32>,
}

impl From<SubtitleConfig> for SubtitleData {
    fn from(config: SubtitleConfig) -> Self {
        SubtitleData {
            id: config.id.into(),
            text: config.text.into(),
            x: config.x,
            y: config.y,
            width: config.width,
            height: config.height,
            background_color: config.background_color.into(),
            text_color: config.text_color.into(),
            font_size: config.font_size,
        }
    }
}

pub struct SubtitleController {
    window: Weak<SubtitleWindow>,
    subtitles: HashMap<String, SubtitleData>,
}

impl SubtitleController {
    /// Constructor
    pub fn new(window: Weak<SubtitleWindow>) -> Self {
        Self {
            window,
            subtitles: HashMap::new(),
        }
    }

    /// Activar/desactivar always-on-top
    pub fn set_always_on_top(&self, enabled: bool) {
        if let Some(window) = self.window.upgrade() {
            window.set_always_on_top_prop(enabled);
        }
    }

    /// Agregar o actualizar subtítulo
    pub fn add_subtitle(&mut self, config: SubtitleConfig) {
        let slint_data = SubtitleData::from(config.clone());
        self.subtitles.insert(config.id, slint_data);
        self.sync();
    }

    /// Eliminar subtítulo
    pub fn remove_subtitle(&mut self, id: &str) {
        if self.subtitles.remove(id).is_some() {
            self.sync();
        }
    }

    /// Actualizar propiedades de un subtítulo
    pub fn update_subtitle(&mut self, id: &str, updates: SubtitleUpdate) {
        if let Some(subtitle) = self.subtitles.get_mut(id) {
            if let Some(text) = updates.text {
                subtitle.text = text.into();
            }
            if let Some(x) = updates.x {
                subtitle.x = x;
            }
            if let Some(y) = updates.y {
                subtitle.y = y;
            }
            if let Some(width) = updates.width {
                subtitle.width = width;
            }
            if let Some(height) = updates.height {
                subtitle.height = height;
            }
            if let Some(bg_color) = updates.background_color {
                subtitle.background_color = bg_color.into();
            }
            if let Some(text_color) = updates.text_color {
                subtitle.text_color = text_color.into();
            }
            if let Some(font_size) = updates.font_size {
                subtitle.font_size = font_size;
            }
            
            self.sync();
        }
    }

    /// Limpiar todos los subtítulos
    pub fn clear_all(&mut self) {
        self.subtitles.clear();
        self.sync();
    }

    /// Obtener todos los subtítulos actuales
    pub fn get_subtitles(&self) -> &HashMap<String, SubtitleData> {
        &self.subtitles
    }

    /// Sincronizar estado con Slint UI
    fn sync(&self) {
        if let Some(window) = self.window.upgrade() {
            // Convertir HashMap a Vec para Slint
            let vec_subtitles: Vec<_> = self.subtitles.values().cloned().collect();
            
            // Establecer en la ventana Slint
            window.set_subtitles(Rc::new(slint::VecModel::from(vec_subtitles)).into());
        }
    }
}
