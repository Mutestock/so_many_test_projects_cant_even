use tauri::InvokeError;

use crate::{
    commands::command_utils::{CommandMessageComposable, SqliteCommandMessage},
    connection::sqlite_connection::get_sqlite_handle,
    model::{model_common::ModelCommon, node_comment::NodeComment},
};

#[tauri::command]
pub async fn cmd_create_node_comment(
    node_name: String,
    comment_content: String,
) -> Result<SqliteCommandMessage<usize>, InvokeError> {
    Ok(SqliteCommandMessage::to_command_message(
        NodeComment::new(node_name, comment_content).create(&get_sqlite_handle()),
    ))
}

#[tauri::command]
pub async fn cmd_update_node_comment_content_by_node_name(
    node_name: String,
    content: String,
) -> Result<SqliteCommandMessage<usize>, InvokeError> {
    Ok(SqliteCommandMessage::to_command_message(
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
) -> Result<SqliteCommandMessage<Option<NodeComment>>, InvokeError> {
    Ok(Option::<NodeComment>::to_command_message(
        NodeComment::read_node_comment_by_node_name(&get_sqlite_handle(), &node_name),
    ))
}

#[tauri::command]
pub async fn cmd_delete_node_comment_by_node_name(
    node_name: String,
) -> Result<SqliteCommandMessage<usize>, InvokeError> {
    Ok(SqliteCommandMessage::to_command_message(
        NodeComment::delete_by_node_name(&get_sqlite_handle(), &node_name),
    ))
}

#[tauri::command]
pub async fn cmd_read_list_node_comment(
) -> Result<SqliteCommandMessage<Vec<NodeComment>>, InvokeError> {
    Ok(Vec::<NodeComment>::to_command_message(
        NodeComment::read_list(&get_sqlite_handle()),
    ))
}
