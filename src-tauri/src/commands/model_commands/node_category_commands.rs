use tauri::InvokeError;

use crate::{
    connection::{connection_common::MindmapConnector, sqlite_connection::SQLITE_CONNECTOR},
    model::{model_common::ModelCommon, node_category::{NodeCategory}},
};

#[tauri::command]
pub async fn cmd_create_node_category(new_category_name: String) -> Result<bool, InvokeError> {
    let node_category = NodeCategory::new(new_category_name);
    let conn = &SQLITE_CONNECTOR.to_owned().connect().unwrap();

    Ok(match node_category.create(&conn) {
        Ok(_) => true,
        Err(_) => false,
    })
}

#[tauri::command]
pub async fn cmd_read_node_category(
    category_name: String,
) -> Result<Option<NodeCategory>, InvokeError> {
    let conn = &SQLITE_CONNECTOR.to_owned().connect().unwrap();

    Ok(NodeCategory::read(&category_name, &conn)
        .expect("Could not execute read node category command"))
}

#[tauri::command]
pub async fn cmd_delete_node_category(node_category: String) -> Result<bool, InvokeError> {
    let conn = &SQLITE_CONNECTOR.to_owned().connect().unwrap();

    Ok(match NodeCategory::delete(&node_category, &conn){
        Ok(_) => true,
        Err(_) => false,
    })
}

#[tauri::command]
pub async fn cmd_read_list_node_category() -> Result<Vec<NodeCategory>, InvokeError> {
    let conn = &SQLITE_CONNECTOR.to_owned().connect().unwrap();
    
    Ok(NodeCategory::read_list(&conn).expect("Could not execute read node category list command"))
}
