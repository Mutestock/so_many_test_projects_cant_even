use tauri::InvokeError;

use crate::{
    commands::command_utils::{CommandResponseComposable, SqliteCommandResponse},
    connection::sqlite_connection::get_sqlite_handle,
    model::{model_common::ModelCommon, tag::Tag},
};

#[tauri::command]
pub fn cmd_create_tag(new_tag_name: String) -> Result<SqliteCommandResponse<usize>, InvokeError> {
    Ok(SqliteCommandResponse::to_command_response(
        Tag::new(new_tag_name).create(&get_sqlite_handle()),
    ))
}

#[tauri::command]
pub fn cmd_read_list_tag() -> Result<SqliteCommandResponse<Vec<Tag>>, InvokeError> {
    Ok(Vec::<Tag>::to_command_response(Tag::read_list(
        &get_sqlite_handle(),
    )))
}

#[tauri::command]
pub fn cmd_delete_tag(tag_name: String) -> Result<SqliteCommandResponse<usize>, InvokeError> {
    Ok(SqliteCommandResponse::to_command_response(Tag::delete(
        &tag_name,
        &get_sqlite_handle(),
    )))
}

#[tauri::command]
pub fn cmd_update_tag(
    old_tag_name: String,
    new_tag_name: String,
) -> Result<SqliteCommandResponse<usize>, InvokeError> {
    Ok(SqliteCommandResponse::to_command_response(
        Tag::update_by_tag_name(&old_tag_name, &new_tag_name, &get_sqlite_handle()),
    ))
}
