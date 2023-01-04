use tauri::InvokeError;

use crate::{
    commands::command_utils::{CommandMessageComposable, SqliteCommandMessage},
    connection::sqlite_connection::get_sqlite_handle,
    model::{model_common::ModelCommon, node_category::NodeCategory},
};

#[tauri::command]
pub async fn cmd_create_node_category(
    new_category_name: String,
) -> Result<SqliteCommandMessage<usize>, InvokeError> {
    Ok(SqliteCommandMessage::to_command_message(
        NodeCategory::new(new_category_name).create(&get_sqlite_handle()),
    ))
}

#[tauri::command]
pub async fn cmd_read_node_category(
    category_name: String,
) -> Result<SqliteCommandMessage<Option<NodeCategory>>, InvokeError> {
    Ok(Option::<NodeCategory>::to_command_message(
        NodeCategory::read(&category_name, &get_sqlite_handle()),
    ))
}

#[tauri::command]
pub async fn cmd_delete_node_category(
    node_category: String,
) -> Result<SqliteCommandMessage<usize>, InvokeError> {
    Ok(SqliteCommandMessage::to_command_message(
        NodeCategory::delete(&node_category, &get_sqlite_handle()),
    ))
}

#[tauri::command]
pub async fn cmd_read_list_node_category(
) -> Result<SqliteCommandMessage<Vec<NodeCategory>>, InvokeError> {
    Ok(Vec::<NodeCategory>::to_command_message(
        NodeCategory::read_list(&get_sqlite_handle()),
    ))
}
