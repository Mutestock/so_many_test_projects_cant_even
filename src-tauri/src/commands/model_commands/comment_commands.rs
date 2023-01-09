use tauri::InvokeError;

use crate::{
    commands::command_utils::{CommandResponseComposable, SqliteCommandResponse},
    connection::sqlite_connection::get_sqlite_handle,
    model::{model_common::ModelCommon, comment::Comment},
};

#[tauri::command]
pub async fn cmd_create_comment(
    node_name: String,
    comment_content: String,
) -> Result<SqliteCommandResponse<usize>, InvokeError> {
    Ok(SqliteCommandResponse::to_command_response(
        Comment::new(node_name, comment_content).create(&get_sqlite_handle()),
    ))
}

#[tauri::command]
pub async fn cmd_update_comment_content_by_node_name(
    node_name: String,
    content: String,
) -> Result<SqliteCommandResponse<usize>, InvokeError> {
    Ok(SqliteCommandResponse::to_command_response(
        Comment::update_comment_content_by_node_name(
            &get_sqlite_handle(),
            &node_name,
            &content,
        ),
    ))
}

#[tauri::command]
pub async fn cmd_read_comment_by_node_name(
    node_name: String,
) -> Result<SqliteCommandResponse<Option<Comment>>, InvokeError> {
    Ok(Option::<Comment>::to_command_response(
        Comment::read_comment_by_node_name(&get_sqlite_handle(), &node_name),
    ))
}

#[tauri::command]
pub async fn cmd_delete_comment_by_node_name(
    node_name: String,
) -> Result<SqliteCommandResponse<usize>, InvokeError> {
    Ok(SqliteCommandResponse::to_command_response(
        Comment::delete_by_node_name(&get_sqlite_handle(), &node_name),
    ))
}

#[tauri::command]
pub async fn cmd_read_list_comment(
) -> Result<SqliteCommandResponse<Vec<Comment>>, InvokeError> {
    Ok(Vec::<Comment>::to_command_response(
        Comment::read_list(&get_sqlite_handle()),
    ))
}
