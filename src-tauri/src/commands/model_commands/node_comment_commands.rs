use tauri::InvokeError;

use crate::{
    commands::command_utils::{CommandResponseComposable, SqliteCommandResponse},
    connection::sqlite_connection::get_sqlite_handle,
    model::{model_common::ModelCommon, node_comment::NodeComment},
};

#[tauri::command]
pub async fn cmd_create_node_comment(
    node_name: String,
    comment_content: String,
) -> Result<SqliteCommandResponse<usize>, InvokeError> {
    Ok(SqliteCommandResponse::to_command_response(
        NodeComment::new(node_name, comment_content).create(&get_sqlite_handle()),
    ))
}

#[tauri::command]
pub async fn cmd_update_node_comment_content_by_node_name(
    node_name: String,
    content: String,
) -> Result<SqliteCommandResponse<usize>, InvokeError> {
    Ok(SqliteCommandResponse::to_command_response(
        NodeComment::update_node_comment_content_by_node_name(
            &get_sqlite_handle(),
            &node_name,
            &content,
        ),
    ))
}

#[tauri::command]
pub async fn cmd_read_node_comment_by_node_name(
    node_name: String,
) -> Result<SqliteCommandResponse<Option<NodeComment>>, InvokeError> {
    Ok(Option::<NodeComment>::to_command_response(
        NodeComment::read_node_comment_by_node_name(&get_sqlite_handle(), &node_name),
    ))
}

#[tauri::command]
pub async fn cmd_delete_node_comment_by_node_name(
    node_name: String,
) -> Result<SqliteCommandResponse<usize>, InvokeError> {
    Ok(SqliteCommandResponse::to_command_response(
        NodeComment::delete_by_node_name(&get_sqlite_handle(), &node_name),
    ))
}

#[tauri::command]
pub async fn cmd_read_list_node_comment(
) -> Result<SqliteCommandResponse<Vec<NodeComment>>, InvokeError> {
    Ok(Vec::<NodeComment>::to_command_response(
        NodeComment::read_list(&get_sqlite_handle()),
    ))
}
