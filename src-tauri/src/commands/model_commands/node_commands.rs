use crate::{
    connection::{connection_common::MindmapConnector, sqlite_connection::SQLITE_CONNECTOR},
    model::{model_common::ModelCommon, node::Node},
};
use tauri::InvokeError;

#[tauri::command]
pub async fn cmd_create_node(node_category: String, name: String) -> Result<bool, InvokeError> {
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
            .expect("Could not execute read node command"),
    )
}

#[tauri::command]
pub async fn cmd_delete_node(node_name: String) -> Result<bool, InvokeError> {
    Ok(
        match Node::delete(&node_name, &SQLITE_CONNECTOR.to_owned().connect().unwrap()) {
            Ok(_) => true,
            Err(_) => false,
        },
    )
}

#[tauri::command]
pub async fn cmd_read_list_node() -> Result<Vec<Node>, InvokeError> {
    let node_list = Node::read_list(&SQLITE_CONNECTOR.to_owned().connect().unwrap())
        .expect("Could not read list node");

    Ok(node_list)
}

#[tauri::command]
pub async fn cmd_read_nodes_by_node_category(
    node_category: String,
) -> Result<Vec<Node>, InvokeError> {
    let node_list = Node::read_nodes_by_node_category(
        &SQLITE_CONNECTOR.to_owned().connect().unwrap(),
        &node_category,
    )
    .expect("Could not read nodes by node category");

    Ok(node_list)
}
