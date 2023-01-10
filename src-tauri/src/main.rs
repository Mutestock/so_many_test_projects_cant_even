#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use mindmap::commands::model_commands::{
    category_commands::*, comment_commands::*, image_commands::*, node_commands::*, tag_commands::*,
};
use mindmap::commands::{command_utils::*, sqlite_commands::*};
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
            cmd_create_node,
            cmd_sqlite_ping,
            cmd_read_node,
            cmd_create_image,
            cmd_read_image,
            cmd_delete_image,
            cmd_read_list_image,
            cmd_create_category,
            cmd_read_category,
            cmd_delete_category,
            cmd_read_list_category,
            cmd_create_comment,
            cmd_read_comment_by_node_name,
            cmd_update_comment_content_by_node_name,
            cmd_delete_comment_by_node_name,
            cmd_read_list_comment,
            cmd_read_list_node,
            cmd_read_nodes_by_category,
            cmd_log,
            cmd_category_toggle_visibility,
            cmd_read_list_toggled_on,
            cmd_create_tag,
            cmd_read_list_tag,
            cmd_delete_tag,
            cmd_update_tag
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
