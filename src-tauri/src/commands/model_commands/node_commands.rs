use crate::{
    connection::{connection_common::MindmapConnector, sqlite_connection::SQLITE_CONNECTOR},
    model::{model_common::ModelCommon, node::Node, node_comment::NodeComment},
};
use tauri::InvokeError;

#[tauri::command]
pub async fn cmd_new_node(node_category: String, name: String) -> Result<bool, InvokeError> {
    Ok(
        match Node::new(name, node_category).create(&SQLITE_CONNECTOR.to_owned().connect().unwrap())
        {
            Ok(_) => true,
            Err(_) => false,
        },
    )
}

#[tauri::command]
pub async fn cmd_read_node(name: String) -> Result<Option<Node>, InvokeError> {
    Ok(
        Node::read(&name, &SQLITE_CONNECTOR.to_owned().connect().unwrap())
            .expect("Could not read node"),
    )
}

#[tauri::command]
pub async fn cmd_append_comment_to_node(
    node_name: String,
    comment_content: String,
) -> Result<bool, InvokeError> {
    Ok(
        match (NodeComment::new(node_name, comment_content))
            .create(&SQLITE_CONNECTOR.to_owned().connect().unwrap())
        {
            Ok(_) => true,
            Err(_) => false,
        },
    )
}
