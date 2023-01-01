use tauri::InvokeError;

use crate::{
    connection::{connection_common::MindmapConnector, sqlite_connection::SQLITE_CONNECTOR},
    model::{model_common::ModelCommon, node_comment::NodeComment},
};

#[tauri::command]
pub async fn cmd_create_node_comment(
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

#[tauri::command]
pub async fn cmd_update_node_comment_content_by_node_name(
    node_name: String,
    content: String,
) -> Result<bool, InvokeError> {
    let conn = &SQLITE_CONNECTOR.to_owned().connect().unwrap();

    Ok(
        match NodeComment::update_node_comment_content_by_node_name(&conn, &node_name, &content) {
            Ok(_) => true,
            Err(_) => false,
        },
    )
}

#[tauri::command]
pub async fn cmd_read_node_comment_by_node_name(
    node_name: String,
) -> Result<Option<NodeComment>, InvokeError> {
    let conn = &SQLITE_CONNECTOR.to_owned().connect().unwrap();
    Ok(
        NodeComment::read_node_comment_by_node_name(&conn, &node_name)
            .expect("Could not execute read node by node name command"),
    )
}

#[tauri::command]
pub async fn cmd_delete_node_comment_by_node_name(node_name: String) -> Result<bool, InvokeError> {
    let conn = &SQLITE_CONNECTOR.to_owned().connect().unwrap();

    Ok(match NodeComment::delete_by_node_name(&conn, &node_name) {
        Ok(_) => true,
        Err(_) => false,
    })
}

#[tauri::command]
pub async fn cmd_read_list_node_comment() -> Result<Vec<NodeComment>, InvokeError> {
    let conn = &SQLITE_CONNECTOR.to_owned().connect().unwrap();
    Ok(NodeComment::read_list(&conn).expect("Could not execute read node command list command"))
}
