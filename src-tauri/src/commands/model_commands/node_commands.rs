use crate::{
    commands::command_utils::{CommandResponseComposable, SqliteCommandResponse},
    connection::sqlite_connection::get_sqlite_handle,
    misc::logging::{log, LogLayer, LogLevel},
    model::{model_common::ModelCommon, node::Node},
};
use tauri::InvokeError;

#[tauri::command]
pub async fn cmd_create_node(
    category: String,
    name: String,
) -> Result<SqliteCommandResponse<usize>, InvokeError> {
    log(
        LogLayer::Backend,
        LogLevel::Info,
        &format!("Node created - Name: {}", &name),
    );
    Ok(SqliteCommandResponse::to_command_response(
        Node::new(name, category).create(&get_sqlite_handle()),
    ))
}

#[tauri::command]
pub async fn cmd_read_node(
    name: String,
) -> Result<SqliteCommandResponse<Option<Node>>, InvokeError> {
    Ok(Option::<Node>::to_command_response(Node::read(
        &name,
        &get_sqlite_handle(),
    )))
}

#[tauri::command]
pub async fn cmd_delete_node(name: String) -> Result<SqliteCommandResponse<usize>, InvokeError> {
    Ok(SqliteCommandResponse::to_command_response(Node::delete(
        &name,
        &get_sqlite_handle(),
    )))
}

#[tauri::command]
pub async fn cmd_read_list_node() -> Result<SqliteCommandResponse<Vec<Node>>, InvokeError> {
    Ok(Vec::<Node>::to_command_response(Node::read_list(
        &get_sqlite_handle(),
    )))
}

#[tauri::command]
pub async fn cmd_read_nodes_by_category(
    category: String,
) -> Result<SqliteCommandResponse<Vec<Node>>, InvokeError> {
    Ok(Vec::<Node>::to_command_response(
        Node::read_nodes_by_category(&get_sqlite_handle(), &category),
    ))
}

#[tauri::command]
pub async fn cmd_read_list_toggled_on() -> Result<SqliteCommandResponse<Vec<Node>>, InvokeError> {
    Ok(Vec::<Node>::to_command_response(
        Node::read_list_toggled_on(&get_sqlite_handle()),
    ))
}
