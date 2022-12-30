#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use mindmap::commands::{node_commands::*, sqlite_commands::*};
use mindmap::connection::{
    connection_common::MindmapConnector, initialize, sqlite_connection::SQLITE_CONNECTOR,
};
use mindmap::misc::directories::create_directories;
use mindmap::ui::menu::{create_menu, handle_menu_event};

fn main() {
    tauri::Builder::default()
        .menu(create_menu())
        .on_menu_event(|event| handle_menu_event(event))
        .invoke_handler(tauri::generate_handler![
            cmd_new_node,
            cmd_append_comment_to_node,
            cmd_sqlite_ping,
            cmd_read_node,
        ])
        .setup(|_| {
            SQLITE_CONNECTOR
                .create_dir_path()
                .expect("Directory path creation for sqlite failed");

            create_directories(false).expect("Could not create directories");

            initialize(&SQLITE_CONNECTOR.connect().unwrap()).expect("Initialization failed");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
