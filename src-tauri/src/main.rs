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

use commands::{node_commands::*, rocks_basic_commands::*, sqlite_commands::*};
use connection::{
    connection_common::MindmapConnector,
    sqlite_connection::{init_pop, SQLITE_CONNECTION},
};
use ui::menu::{create_menu, handle_menu_event};

fn main() {
    tauri::Builder::default()
        .menu(create_menu())
        .on_menu_event(|event| handle_menu_event(event))
        .invoke_handler(tauri::generate_handler![
            ping,
            cmd_rocks_get,
            cmd_rocks_put,
            cmd_new_node,
            cmd_append_comment_to_node,
            cmd_sqlite_ping
        ])
        .setup(|_| {
            SQLITE_CONNECTION
                .create_dir_path()
                .expect("Directory path creation for sqlite failed");
            init_pop().expect("Init pop failed");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
