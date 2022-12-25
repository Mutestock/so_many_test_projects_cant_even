#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate lazy_static;

mod commands;
mod connection;
mod errors;
mod model;
mod state;
mod ui;

use commands::{node_commands::*, sqlite_commands::*};
use connection::{connection_common::MindmapConnector, sqlite_connection::SQLITE_CONNECTION};
use model::{
    model_common::ModelCommon, node::Node, node_category::NodeCategory, node_comment::NodeComment,
};
use ui::menu::{create_menu, handle_menu_event};

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
            SQLITE_CONNECTION
                .create_dir_path()
                .expect("Directory path creation for sqlite failed");

            NodeCategory::init_script(SQLITE_CONNECTION.to_owned())
                .expect("NodeCategory Init script failed");
            Node::init_script(SQLITE_CONNECTION.to_owned())
                .expect("Node Init script failed");
            NodeComment::init_script(SQLITE_CONNECTION.to_owned())
                .expect("NodeComment Init script failed");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
