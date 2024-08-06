// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod inihandler;

use std::env;
use tauri::Manager;

fn main() {
    // Initialize the Tauri application
    tauri::Builder::default()
        // Set up the application
        .setup(move |app| {
            // // If DevTools are enabled and we're in debug mode, open them for the main window
            #[cfg(debug_assertions)]
            if should_enable_devtools() {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                println!("DevTools enabled");
            } else {
                app.get_window("main").unwrap();
            }
            Ok(())
        })
        // Register the invoke handlers for various INI file operations
        .invoke_handler(tauri::generate_handler![
            inihandler::read_ini_file,
            inihandler::write_ini_file,
            inihandler::ensure_ini_files_exist
        ])
        // Run the Tauri application
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Function to determine if DevTools should be enabled
fn should_enable_devtools() -> bool {
    // Check for the ENABLE_DEBUG environment variable
    if let Ok(value) = env::var("ENABLE_DEBUG") {
        // If the environment variable is set, enable DevTools if its value is "true" (case-insensitive)
        value.to_lowercase() == "true"
    } else {
        // If the environment variable is not set, DevTools are disabled by default
        false
    }
}
