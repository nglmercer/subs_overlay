// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::Arc;

// Include the generated Slint code
slint::include_modules!();

mod controller;
mod color_utils;
mod api_server;
mod mcp_server;
mod config;

use controller::{SubtitleController, SubtitleConfig, SubtitleUpdate};
use api_server::{ApiState, add_subtitle, update_subtitle, remove_subtitle, list_subtitles, clear_all_subtitles, toggle_click_through, get_status, with_cors};

// Global state for click-through toggle
static mut CLICK_THROUGH_ENABLED: bool = true;

#[cfg(target_os = "windows")]
pub fn set_click_through(enabled: bool) -> Result<(), Box<dyn std::error::Error>> {
    use windows::Win32::UI::WindowsAndMessaging::{SetWindowLongPtrW, GetWindowLongPtrW, GWL_EXSTYLE, WS_EX_TRANSPARENT, WS_EX_LAYERED};
    
    unsafe {
        CLICK_THROUGH_ENABLED = enabled;
        let hwnd = windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow();
        let mut ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
        
        if enabled {
            ex_style |= WS_EX_TRANSPARENT.0 as isize;
            ex_style |= WS_EX_LAYERED.0 as isize;
        } else {
            ex_style &= !WS_EX_TRANSPARENT.0 as isize;
        }
        
        SetWindowLongPtrW(hwnd, GWL_EXSTYLE, ex_style);
        
        println!("Click-through: {}", if enabled { "ON" } else { "OFF" });
    }
    Ok(())
}

#[cfg(target_os = "windows")]
#[allow(dead_code)]
fn toggle_click_through_local() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        set_click_through(!CLICK_THROUGH_ENABLED)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Cli = clap::Parser::parse();

    match args.command {
        Commands::Gui => run_gui().await?,
        Commands::Api { port } => run_api(port).await?,
        Commands::Mcp => run_mcp().await?,
    }

    Ok(())
}

async fn run_gui() -> Result<(), Box<dyn Error>> {
    // Crear ventana Slint
    let ui = SubtitleWindow::new()?;
    let ui_weak = ui.as_weak();
    
    // Crear controller
    let controller = Rc::new(RefCell::new(SubtitleController::new(ui_weak)));
    
    // Configuración inicial
    controller.borrow().set_always_on_top(true);
    
    // Habilitar click-through (solo Windows)
    #[cfg(target_os = "windows")]
    set_click_through(true)?;
    
    // Agregar subtítulos de ejemplo
    controller.borrow_mut().add_subtitle(SubtitleConfig {
        id: "sub1".to_string(),
        text: "¡Hola desde Rust + Slint!".to_string(),
        x: 100.0,
        y: 50.0,
        width: 400.0,
        height: 60.0,
        background_color: "#CC000000".to_string(),
        text_color: "#FFFFFFFF".to_string(),
        font_size: 24.0,
    });
    
    controller.borrow_mut().add_subtitle(SubtitleConfig {
        id: "sub2".to_string(),
        text: "Este es un segundo subtítulo de ejemplo".to_string(),
        x: 150.0,
        y: 130.0,
        width: 350.0,
        height: 50.0,
        background_color: "#AAFF0000".to_string(),
        text_color: "#FFFFFF00".to_string(),
        font_size: 18.0,
    });
    
    // Programar actualización después de 3 segundos
    let controller_clone = controller.clone();
    slint::Timer::single_shot(std::time::Duration::from_secs(3), move || {
        let mut ctrl = controller_clone.borrow_mut();
        ctrl.update_subtitle("sub1", SubtitleUpdate {
            text: Some("¡Texto actualizado después de 3 segundos!".to_string()),
            y: Some(80.0),
            ..Default::default()
        });
    });
    
    // Programar eliminación después de 6 segundos
    let controller_clone = controller.clone();
    slint::Timer::single_shot(std::time::Duration::from_secs(6), move || {
        let mut ctrl = controller_clone.borrow_mut();
        ctrl.remove_subtitle("sub2");
    });
    
    // Programar nuevo subtítulo después de 8 segundos
    let controller_clone = controller.clone();
    slint::Timer::single_shot(std::time::Duration::from_secs(8), move || {
        let mut ctrl = controller_clone.borrow_mut();
        ctrl.add_subtitle(SubtitleConfig {
            id: "sub3".to_string(),
            text: "Nuevo subtítulo agregado dinámicamente".to_string(),
            x: 200.0,
            y: 200.0,
            width: 300.0,
            height: 40.0,
            background_color: "#CC00FF00".to_string(),
            text_color: "#000000".to_string(),
            font_size: 16.0,
        });
    });
    
    // Ejecutar aplicación
    ui.run()?;
    
    Ok(())
}

async fn run_api(port: u16) -> Result<(), Box<dyn Error>> {
    println!("Starting API server on port {}", port);
    
    // Create a dummy controller for API mode (in real implementation, this would connect to GUI)
    let ui = SubtitleWindow::new()?;
    let dummy_controller = SubtitleController::new(ui.as_weak());
    let state = Arc::new(ApiState::new(dummy_controller));

    // API routes with proper warp syntax
    use warp::Filter;
    let state_filter = warp::any().map(move || state.clone());

    let add_subtitle_route = warp::path("subtitles")
        .and(warp::post())
        .and(warp::body::json())
        .and(state_filter.clone())
        .and_then(add_subtitle);

    let update_subtitle_route = warp::path!("subtitles" / String)
        .and(warp::put())
        .and(warp::body::json())
        .and(state_filter.clone())
        .and_then(update_subtitle);

    let remove_subtitle_route = warp::path!("subtitles" / String)
        .and(warp::delete())
        .and(state_filter.clone())
        .and_then(remove_subtitle);

    let list_subtitles_route = warp::path("subtitles")
        .and(warp::get())
        .and(state_filter.clone())
        .and_then(list_subtitles);

    let clear_all_route = warp::path("subtitles")
        .and(warp::delete())
        .and(state_filter.clone())
        .and_then(clear_all_subtitles);

    let toggle_click_route = warp::path("window")
        .and(warp::path("toggle-clickthrough"))
        .and(warp::post())
        .and(state_filter.clone())
        .and_then(toggle_click_through);

    let status_route = warp::path("status")
        .and(warp::get())
        .and(state_filter.clone())
        .and_then(get_status);

    let routes = add_subtitle_route
        .or(update_subtitle_route)
        .or(remove_subtitle_route)
        .or(list_subtitles_route)
        .or(clear_all_route)
        .or(toggle_click_route)
        .or(status_route)
        .with(with_cors())
        .with(warp::log("api"));

    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
    
    Ok(())
}

async fn run_mcp() -> Result<(), Box<dyn Error>> {
    println!("Starting MCP server...");
    
    // Initialize MCP server
    let init_response = mcp_server::initialize_mcp_server();
    println!("{}", serde_json::to_string_pretty(&init_response)?);

    // In a real implementation, this would handle stdin/stdout communication
    // For now, just print the available tools
    let tools = mcp_server::get_mcp_tools();
    for tool in tools {
        println!("Available tool: {} - {}", tool.name, tool.description);
    }

    Ok(())
}

#[derive(clap::Parser)]
#[command(name = "subtitle-overlay")]
#[command(about = "A subtitle overlay system with GUI, API, and MCP support")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Run the GUI application
    Gui,
    /// Start the REST API server
    Api {
        /// Port to listen on
        #[arg(short, long, default_value = "8080")]
        port: u16,
    },
    /// Start the MCP (Model Context Protocol) server
    Mcp,
}
