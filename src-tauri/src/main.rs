#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate lazy_static;


mod commands;
mod connection;
mod state;
mod ui;

use commands::rocks_basic_commands::ping;


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, ping])
        .setup(|app| {
            app
                .path_resolver()
                .resolve_resource("rocks")
                .expect("Couldn't get rocks resource");
                Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
