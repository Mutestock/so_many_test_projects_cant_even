#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod connection;
mod state;
mod ui;
mod model;
mod errors;

use commands::rocks_basic_commands::*;

use crate::commands::node_commands::{cmd_new_node, cmd_append_comment_to_node};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            ping,
            cmd_rocks_get,
            cmd_rocks_put,
            cmd_new_node,
            cmd_append_comment_to_node
        ])
        .setup(|app| {
            app.path_resolver()
                .resolve_resource("rocks")
                .expect("Couldn't get rocks resource");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
