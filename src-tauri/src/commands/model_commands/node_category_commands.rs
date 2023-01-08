use tauri::InvokeError;

use crate::{
    commands::command_utils::{CommandResponseComposable, SqliteCommandResponse},
    connection::sqlite_connection::get_sqlite_handle,
    model::{model_common::ModelCommon, node_category::NodeCategory},
};

#[tauri::command]
pub async fn cmd_create_node_category(
    new_category_name: String,
    color_code_hex: String,
) -> Result<SqliteCommandResponse<usize>, InvokeError> {
    if !NodeCategory::is_valid_hex(&color_code_hex) {
        return Ok(SqliteCommandResponse::to_command_response_with_message(
            NodeCategory::new(new_category_name, color_code_hex).create(&get_sqlite_handle()),
            "Color hex code was invalid".to_owned(),
        ));
    }

    Ok(SqliteCommandResponse::to_command_response(
        NodeCategory::new(new_category_name, color_code_hex).create(&get_sqlite_handle()),
    ))
}

#[tauri::command]
pub async fn cmd_read_node_category(
    category_name: String,
) -> Result<SqliteCommandResponse<Option<NodeCategory>>, InvokeError> {
    Ok(Option::<NodeCategory>::to_command_response(
        NodeCategory::read(&category_name, &get_sqlite_handle()),
    ))
}

#[tauri::command]
pub async fn cmd_delete_node_category(
    node_category: String,
) -> Result<SqliteCommandResponse<usize>, InvokeError> {
    Ok(SqliteCommandResponse::to_command_response(
        NodeCategory::delete(&node_category, &get_sqlite_handle()),
    ))
}

#[tauri::command]
pub async fn cmd_read_list_node_category(
) -> Result<SqliteCommandResponse<Vec<NodeCategory>>, InvokeError> {
    Ok(Vec::<NodeCategory>::to_command_response(
        NodeCategory::read_list(&get_sqlite_handle()),
    ))
}


#[tauri::command]
pub async fn cmd_category_toggle_visibility(
    node_category: String,
) -> Result<SqliteCommandResponse<usize>, InvokeError> {
    Ok(SqliteCommandResponse::to_command_response(
        NodeCategory::delete(&node_category, &get_sqlite_handle()),
    ))
}
