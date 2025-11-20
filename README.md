# Subs Overlay Library

Una librería Rust para crear overlays de texto transparentes con capacidad de ignorar input del mouse (input passthrough), siempre visibles (always on top) y con sistema de registro para manejar múltiples ventanas.

## Características

- **Texto transparente**: Muestra texto en pantalla con fondo transparente
- **Ignorar input**: Los clics del mouse pasan a través del overlay a las aplicaciones de fondo
- **Siempre visible**: Los overlays se mantienen sobre todas las demás ventanas
- **Gestión múltiple**: Registra y maneja múltiples overlays simultáneamente
- **API simple**: Fácil de integrar en otros programas

## Instalación

Agrega esto a tu `Cargo.toml`:

```toml
[dependencies]
subs_overlay_lib = "0.1.0"
```

## Uso Básico

### Crear un overlay simple

```rust
use subs_overlay_lib::{create_text_overlay, update_overlay_text, remove_overlay};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Crear un overlay simple
    let overlay_id = create_text_overlay(
        "Hola, Mundo!", // Texto
        100,            // Posición X
        100,            // Posición Y
        300,            // Ancho
        100             // Alto
    )?;
    
    // Actualizar el texto del overlay
    update_overlay_text(&overlay_id, "Texto actualizado")?;
    
    // Eliminar el overlay
    remove_overlay(&overlay_id)?;
    
    Ok(())
}
```

### Uso avanzado con OverlayManager

```rust
use subs_overlay_lib::{OverlayManager, OverlayConfig, TextConfig};
use std::sync::{Arc, Mutex};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Crear un gestor de overlays
    let manager = Arc::new(Mutex::new(OverlayManager::new()));
    
    // Configurar el texto
    let text_config = TextConfig {
        content: "Overlay personalizado".to_string(),
        font_size: 24.0,
        color: "#FF0000".to_string(), // Rojo
        position: (200, 200),
    };
    
    // Configurar el overlay
    let overlay_config = OverlayConfig {
        text: text_config,
        width: 400,
        height: 80,
        transparent: true,
        always_on_top: true,
        ignore_input: true,
    };
    
    // Crear y mostrar el overlay
    let overlay_id = {
        let manager = manager.lock().unwrap();
        manager.create_overlay(overlay_config)?
    };
    {
        let manager = manager.lock().unwrap();
        manager.show_overlay(&overlay_id)?;
    }
    
    // Listar todos los overlays activos
    {
        let manager = manager.lock().unwrap();
        let overlays = manager.list_overlays();
        println!("Overlays activos: {:?}", overlays);
    }
    
    Ok(())
}
```

## API Reference

### OverlayManager

Gestiona múltiples overlays y proporciona métodos para crear, modificar y eliminarlos.

#### Métodos

- `new()` -> Crea un nuevo gestor de overlays
- `create_overlay(config: OverlayConfig)` -> Crea un nuevo overlay
- `show_overlay(overlay_id: &OverlayId)` -> Muestra un overlay existente
- `hide_overlay(overlay_id: &OverlayId)` -> Oculta un overlay
- `update_text(overlay_id: &OverlayId, text: &str)` -> Actualiza el texto
- `update_position(overlay_id: &OverlayId, x: i32, y: i32)` -> Actualiza la posición
- `remove_overlay(overlay_id: &OverlayId)` -> Elimina un overlay
- `list_overlays()` -> Lista todos los IDs de overlays activos
- `get_overlay_config(overlay_id: &OverlayId)` -> Obtiene la configuración de un overlay

### Estructuras de Configuración

#### TextConfig

Configuración para el texto del overlay:

- `content: String` - El texto a mostrar
- `font_size: f32` - Tamaño de fuente en píxeles
- `color: String` - Color del texto en formato #AARRGGBB o #RRGGBB
- `position: (i32, i32)` - Posición (x, y) en pantalla

#### OverlayConfig

Configuración completa del overlay:

- `text: TextConfig` - Configuración del texto
- `width: i32` - Ancho de la ventana en píxeles
- `height: i32` - Alto de la ventana en píxeles
- `transparent: bool` - Si la ventana debe ser transparente
- `always_on_top: bool` - Si la ventana debe estar siempre encima
- `ignore_input: bool` - Si la ventana debe ignorar el input del mouse

### Funciones de Conveniencia

- `create_text_overlay(text, x, y, width, height)` - Crea un overlay simple con valores por defecto
- `update_overlay_text(overlay_id, text)` - Actualiza el texto de un overlay
- `remove_overlay(overlay_id)` - Elimina un overlay

## Sistema de Registro

La librería utiliza un sistema de registro basado en UUIDs para identificar cada overlay de manera única. Cada vez que creas un overlay, se genera un ID único que puedes usar para referenciarlo posteriormente.

```rust
use subs_overlay_lib::OverlayManager;
use std::sync::{Arc, Mutex};

let manager = Arc::new(Mutex::new(OverlayManager::new()));

// Crear múltiples overlays
let id1 = {
    let manager = manager.lock().unwrap();
    manager.create_overlay(config1)?
};
let id2 = {
    let manager = manager.lock().unwrap();
    manager.create_overlay(config2)?
};

// Listar todos los overlays
{
    let manager = manager.lock().unwrap();
    let ids = manager.list_overlays();
    // ids contendrá [id1, id2]
}
```

## Ejemplos de Aplicación

### Sistema de Subtítulos Personalizado

```rust
use subs_overlay_lib::{OverlayManager, OverlayConfig, TextConfig};
use std::sync::{Arc, Mutex};

// Función para mostrar subtítulos
fn show_subtitle(text: &str, manager: &Arc<Mutex<OverlayManager>>) -> Result<String, Box<dyn std::error::Error>> {
    let text_config = TextConfig {
        content: text.to_string(),
        font_size: 28.0,
        color: "#FFFFFF".to_string(),
        position: (100, 850), // Parte inferior de la pantalla
    };
    
    let overlay_config = OverlayConfig {
        text: text_config,
        width: 1200,
        height: 100,
        transparent: true,
        always_on_top: true,
        ignore_input: true,
    };
    
    let manager = manager.lock().unwrap();
    manager.create_overlay(overlay_config)
}

// Mostrar subtítulo
let manager = Arc::new(Mutex::new(OverlayManager::new()));
let subtitle_id = show_subtitle("Este es un subtítulo de ejemplo", &manager)?;
```

### Sistema de Notificaciones

```rust
use subs_overlay_lib::{OverlayManager, OverlayConfig, TextConfig};
use std::sync::{Arc, Mutex};

// Función para mostrar notificación
fn show_notification(text: &str, manager: &Arc<Mutex<OverlayManager>>) -> Result<String, Box<dyn std::error::Error>> {
    let text_config = TextConfig {
        content: text.to_string(),
        font_size: 20.0,
        color: "#FFFF00".to_string(), // Amarillo
        position: (1600, 20), // Esquina superior derecha
    };
    
    let overlay_config = OverlayConfig {
        text: text_config,
        width: 300,
        height: 60,
        transparent: true,
        always_on_top: true,
        ignore_input: true,
    };
    
    let manager = manager.lock().unwrap();
    manager.create_overlay(overlay_config)
}
```

## Ejemplos Disponibles

El proyecto incluye varios ejemplos que demuestran cómo usar la librería:

1. **Ejemplo básico**: `cargo run` - Muestra un overlay simple con texto actualizable
2. **Múltiples overlays**: `cargo run --example multiple_overlays` - Gestiona varios overlays simultáneamente
3. **Integración completa**: `cd integration_example && cargo run --bin main` - Ejemplo de integración en una aplicación completa

## Limitaciones y Consideraciones

1. **Plataforma**: Actualmente optimizado para Windows. Para otras plataformas, se necesitarían adaptaciones específicas.
2. **Transparencia y Input Passthrough**: La implementación actual crea ventanas transparentes pero el soporte completo para "ignorar input" y "always on top" requiere acceso al handle nativo de la ventana, lo cual puede variar según la versión de Slint.
3. **Rendimiento**: Para un gran número de overlays (100+), podría ser necesario optimizar el renderizado.
4. **Dependencias**: Requiere el runtime de Slint para el renderizado de la interfaz.

## Implementación Futura

Para una implementación completa de todas las características, se necesitaría:

1. Acceso al handle nativo de la ventana de Slint
2. Integración con APIs específicas de cada plataforma:
   - Windows: SetWindowLong, SetLayeredWindowAttributes
   - Linux: X11 o Wayland específico
   - macOS: Core Graphics y Window Server APIs
3. Sistema de callbacks para eventos del overlay

## Contribuciones

Las contribuciones son bienvenidas. Las áreas de mejora incluyen:

- Soporte multiplataforma (Linux, macOS)
- Optimización para grandes cantidades de overlays
- Sistema de eventos y callbacks
- Más opciones de personalización visual
- Implementación completa de transparencia y input passthrough

## Licencia

MIT License - ver el archivo LICENSE para más detalles.