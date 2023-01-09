use tauri::InvokeError;

use crate::{
    commands::command_utils::{CommandResponseComposable, SqliteCommandResponse},
    connection::sqlite_connection::get_sqlite_handle,
    model::{model_common::ModelCommon, image::Image},
};

#[tauri::command]
pub async fn cmd_create_image(
    image_title: String,
    node_name: String,
) -> Result<SqliteCommandResponse<usize>, InvokeError> {
    Ok(SqliteCommandResponse::to_command_response(
        Image::new(image_title, node_name).create(&get_sqlite_handle()),
    ))
}

#[tauri::command]
pub async fn cmd_read_image(
    image_title: String,
) -> Result<SqliteCommandResponse<Option<Image>>, InvokeError> {
    Ok(Option::<Image>::to_command_response(Image::read(
        &image_title,
        &get_sqlite_handle(),
    )))
}

#[tauri::command]
pub async fn cmd_delete_image(
    image_title: String,
) -> Result<SqliteCommandResponse<usize>, InvokeError> {
    Ok(SqliteCommandResponse::to_command_response(Image::delete(
        &image_title,
        &get_sqlite_handle(),
    )))
}

#[tauri::command]
pub async fn cmd_read_list_image() -> Result<SqliteCommandResponse<Vec<Image>>, InvokeError>
{
    Ok(Vec::<Image>::to_command_response(Image::read_list(
        &get_sqlite_handle(),
    )))
}
