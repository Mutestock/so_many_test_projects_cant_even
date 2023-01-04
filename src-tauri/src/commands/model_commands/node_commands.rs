use crate::{
    commands::command_utils::{CommandMessageComposable, SqliteCommandMessage},
    connection::sqlite_connection::get_sqlite_handle,
    model::{model_common::ModelCommon, node::Node},
};
use tauri::InvokeError;

#[tauri::command]
pub async fn cmd_create_node(
    node_category: String,
    name: String,
) -> Result<SqliteCommandMessage<usize>, InvokeError> {
    Ok(SqliteCommandMessage::to_command_message(
        Node::new(name, node_category).create(&get_sqlite_handle()),
    ))
}

#[tauri::command]
pub async fn cmd_read_node(
    name: String,
) -> Result<SqliteCommandMessage<Option<Node>>, InvokeError> {
    Ok(Option::<Node>::to_command_message(Node::read(
        &name,
        &get_sqlite_handle(),
    )))
}

#[tauri::command]
pub async fn cmd_delete_node(
    node_name: String,
) -> Result<SqliteCommandMessage<usize>, InvokeError> {
    Ok(SqliteCommandMessage::to_command_message(Node::delete(
        &node_name,
        &get_sqlite_handle(),
    )))
}

#[tauri::command]
pub async fn cmd_read_list_node() -> Result<SqliteCommandMessage<Vec<Node>>, InvokeError> {
    Ok(Vec::<Node>::to_command_message(Node::read_list(
        &get_sqlite_handle(),
    )))
}

#[tauri::command]
pub async fn cmd_read_nodes_by_node_category(
    node_category: String,
) -> Result<SqliteCommandMessage<Vec<Node>>, InvokeError> {
    Ok(Vec::<Node>::to_command_message(
        Node::read_nodes_by_node_category(&get_sqlite_handle(), &node_category),
    ))
}
