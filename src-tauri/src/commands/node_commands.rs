use crate::{
    connection::sqlite_connection::SQLITE_CONNECTION,
    model::{model_common::ModelCommon, node::Node, node_comment::NodeComment},
};
use tauri::InvokeError;

#[tauri::command]
pub async fn cmd_new_node(node_category: String, name: String) -> Result<bool, InvokeError> {
    Ok(
        match Node::new(name, node_category).create(SQLITE_CONNECTION.to_owned()) {
            Ok(_) => true,
            Err(_) => false,
        },
    )
}

#[tauri::command]
pub async fn cmd_read_node(name: String) -> Result<Node, InvokeError> {
    Ok(Node::read(&name, SQLITE_CONNECTION.to_owned()).expect("Could not read node"))
}

#[tauri::command]
pub async fn cmd_append_comment_to_node(
    node_name: String,
    comment_content: String,
) -> Result<bool, InvokeError> {
    Ok(
        match (NodeComment::new(node_name, comment_content)).create(SQLITE_CONNECTION.to_owned()) {
            Ok(_) => true,
            Err(_) => false,
        },
    )
}
