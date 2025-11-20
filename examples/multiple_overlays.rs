//! Ejemplo de uso de múltiples overlays con la librería subs_overlay

use std::{error::Error, thread, time::Duration};
use subs_overlay_lib::{OverlayManager, OverlayConfig, TextConfig};
use std::sync::{Arc, Mutex};

fn main() -> Result<(), Box<dyn Error>> {
    // Crear un gestor de overlays
    let manager = Arc::new(Mutex::new(OverlayManager::new()));

    // Crear un overlay para notificaciones
    let notification_config = TextConfig {
        content: "Notificación importante".to_string(),
        font_size: 20.0,
        color: "#FFFF00".to_string(), // Amarillo
        position: (1400, 20),
    };

    let notification_overlay_config = OverlayConfig {
        text: notification_config,
        width: 400,
        height: 80,
        transparent: true,
        always_on_top: true,
        ignore_input: true,
    };

    let notification_id = {
        let manager = manager.lock().unwrap();
        manager.create_overlay(notification_overlay_config)?
    };
    {
        let manager = manager.lock().unwrap();
        manager.show_overlay(&notification_id)?;
    }

    // Crear un overlay para subtítulos
    let subtitle_config = TextConfig {
        content: "Este es un ejemplo de subtítulo".to_string(),
        font_size: 24.0,
        color: "#FFFFFF".to_string(), // Blanco
        position: (300, 800),
    };

    let subtitle_overlay_config = OverlayConfig {
        text: subtitle_config,
        width: 800,
        height: 100,
        transparent: true,
        always_on_top: true,
        ignore_input: true,
    };

    let subtitle_id = {
        let manager = manager.lock().unwrap();
        manager.create_overlay(subtitle_overlay_config)?
    };
    {
        let manager = manager.lock().unwrap();
        manager.show_overlay(&subtitle_id)?;
    }

    // Crear un overlay para información del sistema
    let system_info_config = TextConfig {
        content: "CPU: 45% | RAM: 3.2GB/8GB | GPU: 60°C".to_string(),
        font_size: 16.0,
        color: "#00FF00".to_string(), // Verde
        position: (10, 10),
    };

    let system_info_overlay_config = OverlayConfig {
        text: system_info_config,
        width: 500,
        height: 60,
        transparent: true,
        always_on_top: true,
        ignore_input: true,
    };

    let system_info_id = {
        let manager = manager.lock().unwrap();
        manager.create_overlay(system_info_overlay_config)?
    };
    {
        let manager = manager.lock().unwrap();
        manager.show_overlay(&system_info_id)?;
    }

    // Listar todos los overlays activos
    {
        let manager = manager.lock().unwrap();
        let overlays = manager.list_overlays();
        println!("Overlays activos:");
        for id in &overlays {
            println!("  - {}", id);
        }
    }

    // Simular actualización de overlays
    let manager_clone = Arc::clone(&manager);
    let subtitle_id_clone = subtitle_id.clone();
    let system_info_id_clone = system_info_id.clone();

    thread::spawn(move || {
        let mut counter = 1;
        loop {
            thread::sleep(Duration::from_secs(3));

            // Actualizar subtítulo
            let new_subtitle = format!("Este es el subtítulo número {}", counter);
            {
                let manager = manager_clone.lock().unwrap();
                if let Err(e) = manager.update_text(&subtitle_id_clone, &new_subtitle) {
                    eprintln!("Error al actualizar subtítulo: {}", e);
                }
            }

            // Actualizar información del sistema
            let cpu = 40 + (counter % 20);
            let ram = 3.0 + (counter % 4) as f32;
            let gpu = 55 + (counter % 10);
            let new_system_info = format!("CPU: {}% | RAM: {:.1}GB/8GB | GPU: {}°C", cpu, ram, gpu);
            {
                let manager = manager_clone.lock().unwrap();
                if let Err(e) = manager.update_text(&system_info_id_clone, &new_system_info) {
                    eprintln!("Error al actualizar información del sistema: {}", e);
                }
            }

            counter += 1;

            if counter > 10 {
                break;
            }
        }
    });

    println!("Presiona Enter para eliminar los overlays...");
    let _ = std::io::stdin().read_line(&mut String::new());

    // Eliminar overlays
    {
        let manager = manager.lock().unwrap();
        manager.remove_overlay(&notification_id)?;
        manager.remove_overlay(&subtitle_id)?;
        manager.remove_overlay(&system_info_id)?;
    }

    println!("Todos los overlays han sido eliminados.");

    Ok(())
}
