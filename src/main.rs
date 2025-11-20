// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use subs_overlay_lib::{create_text_overlay, remove_overlay, update_overlay_text};

use log::{error, info};

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    info!("Creating a transparent overlay...");

    // Create a simple text overlay using the convenience function
    let overlay_id = create_text_overlay(
        "Hello, World! This is a transparent overlay.",
        200, // x position
        200, // y position
        500, // width
        100, // height
    )?;

    info!("Overlay created with ID: {}", overlay_id);

    // Clone the overlay_id for the thread
    let overlay_id_clone = overlay_id.clone();

    // Spawn a thread to handle user input and overlay updates
    std::thread::spawn(move || {
        let mut counter = 0;
        loop {
            counter += 1;
            std::thread::sleep(std::time::Duration::from_secs(1));

            let text = format!("Counter: {}", counter);
            info!("Updating text to: {}", text);

            // Update the overlay text
            if let Err(e) = update_overlay_text(&overlay_id_clone, &text) {
                error!("Error updating text: {}", e);
            }

            // Exit after 30 seconds for testing purposes
            if counter >= 30 {
                break;
            }
        }

        info!("Counter finished. Removing overlay...");

        // Remove the overlay
        if let Err(e) = remove_overlay(&overlay_id_clone) {
            error!("Error removing overlay: {}", e);
        }
        info!("Overlay removed.");

        // Quit the event loop to exit the application
        if let Err(e) = slint::quit_event_loop() {
            eprintln!("Error quitting event loop: {}", e);
        }
    });

    // Run the Slint event loop on the main thread
    // This blocks until quit_event_loop() is called
    slint::run_event_loop()?;

    Ok(())
}
