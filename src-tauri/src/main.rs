#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use mindmap::commands::model_commands::{
    node_category_commands::*, node_commands::*, node_comment_commands::*, node_image_commands::*,
};
use mindmap::commands::sqlite_commands::*;
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
            cmd_create_node_image,
            cmd_read_node_image,
            cmd_update_node_image,
            cmd_delete_node_image,
            cmd_read_list_node_image,
            cmd_create_node_category,
            cmd_read_node_category,
            cmd_update_node_category,
            cmd_delete_node_category,
            cmd_read_list_node_category,
            cmd_create_node_comment,
            cmd_read_node_comment,
            cmd_update_node_comment,
            cmd_delete_node_comment,
            cmd_read_list_node_comment,
        ])
        .setup(|_| {
            SQLITE_CONNECTOR
                .create_dir_path()
                .expect("Directory path creation for sqlite failed");

            create_directories().expect("Could not create directories");

            initialize(&SQLITE_CONNECTOR.connect().unwrap()).expect("Initialization failed");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
