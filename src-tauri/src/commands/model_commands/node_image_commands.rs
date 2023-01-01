use tauri::InvokeError;

use crate::{
    connection::{connection_common::MindmapConnector, sqlite_connection::SQLITE_CONNECTOR},
    model::{model_common::ModelCommon, node_image::NodeImage},
};

#[tauri::command]
pub async fn cmd_create_node_image(
    image_title: String,
    node_name: String,
) -> Result<bool, InvokeError> {
    let conn = &SQLITE_CONNECTOR.to_owned().connect().unwrap();
    let node_image = NodeImage::new(image_title, node_name);

    Ok(match node_image.create(&conn) {
        Ok(_) => true,
        Err(_) => false,
    })
}

#[tauri::command]
pub async fn cmd_read_node_image(image_title: String) -> Result<Option<NodeImage>, InvokeError> {
    let conn = &SQLITE_CONNECTOR.to_owned().connect().unwrap();
    Ok(NodeImage::read(&image_title, &conn).expect("Could not execute read node image command"))
}

#[tauri::command]
pub async fn cmd_delete_node_image(image_title: String) -> Result<bool, InvokeError> {
    let conn = &SQLITE_CONNECTOR.to_owned().connect().unwrap();
    Ok(match NodeImage::delete(&image_title, &conn) {
        Ok(_) => true,
        Err(_) => false,
    })
}

#[tauri::command]
pub async fn cmd_read_list_node_image() -> Result<Vec<NodeImage>, InvokeError> {
    let conn = &SQLITE_CONNECTOR.to_owned().connect().unwrap();
    Ok(NodeImage::read_list(&conn).expect("Could not execute read node commet list command"))
}
