use tauri::InvokeError;

use crate::{
    commands::command_utils::{CommandMessageComposable, SqliteCommandMessage},
    connection::sqlite_connection::get_sqlite_handle,
    model::{model_common::ModelCommon, node_image::NodeImage},
};

#[tauri::command]
pub async fn cmd_create_node_image(
    image_title: String,
    node_name: String,
) -> Result<SqliteCommandMessage<usize>, InvokeError> {
    Ok(SqliteCommandMessage::to_command_message(
        NodeImage::new(image_title, node_name).create(&get_sqlite_handle()),
    ))
}

#[tauri::command]
pub async fn cmd_read_node_image(
    image_title: String,
) -> Result<SqliteCommandMessage<Option<NodeImage>>, InvokeError> {
    Ok(Option::<NodeImage>::to_command_message(NodeImage::read(
        &image_title,
        &get_sqlite_handle(),
    )))
}

#[tauri::command]
pub async fn cmd_delete_node_image(
    image_title: String,
) -> Result<SqliteCommandMessage<usize>, InvokeError> {
    Ok(SqliteCommandMessage::to_command_message(NodeImage::delete(
        &image_title,
        &get_sqlite_handle(),
    )))
}

#[tauri::command]
pub async fn cmd_read_list_node_image() -> Result<SqliteCommandMessage<Vec<NodeImage>>, InvokeError>
{
    Ok(Vec::<NodeImage>::to_command_message(NodeImage::read_list(
        &get_sqlite_handle(),
    )))
}
