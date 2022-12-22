#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod connection;
mod errors;
mod model;
mod state;
mod ui;

use commands::rocks_basic_commands::*;
use ui::menu::{create_menu, handle_menu_event};

use crate::commands::node_commands::{cmd_append_comment_to_node, cmd_new_node};

fn main() {
    tauri::Builder::default()
        .menu(create_menu())
        .on_menu_event(|event| handle_menu_event(event))
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
