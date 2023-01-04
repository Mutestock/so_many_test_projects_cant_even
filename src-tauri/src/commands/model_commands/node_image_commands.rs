use tauri::InvokeError;

use crate::{
    commands::command_utils::{CommandResponseComposable, SqliteCommandResponse},
    connection::sqlite_connection::get_sqlite_handle,
    model::{model_common::ModelCommon, node_image::NodeImage},
};

#[tauri::command]
pub async fn cmd_create_node_image(
    image_title: String,
    node_name: String,
) -> Result<SqliteCommandResponse<usize>, InvokeError> {
    Ok(SqliteCommandResponse::to_command_response(
        NodeImage::new(image_title, node_name).create(&get_sqlite_handle()),
    ))
}

#[tauri::command]
pub async fn cmd_read_node_image(
    image_title: String,
) -> Result<SqliteCommandResponse<Option<NodeImage>>, InvokeError> {
    Ok(Option::<NodeImage>::to_command_response(NodeImage::read(
        &image_title,
        &get_sqlite_handle(),
    )))
}

#[tauri::command]
pub async fn cmd_delete_node_image(
    image_title: String,
) -> Result<SqliteCommandResponse<usize>, InvokeError> {
    Ok(SqliteCommandResponse::to_command_response(NodeImage::delete(
        &image_title,
        &get_sqlite_handle(),
    )))
}

#[tauri::command]
pub async fn cmd_read_list_node_image() -> Result<SqliteCommandResponse<Vec<NodeImage>>, InvokeError>
{
    Ok(Vec::<NodeImage>::to_command_response(NodeImage::read_list(
        &get_sqlite_handle(),
    )))
}
