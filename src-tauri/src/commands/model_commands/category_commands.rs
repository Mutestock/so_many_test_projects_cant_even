use tauri::InvokeError;

use crate::{
    commands::command_utils::{CommandResponseComposable, SqliteCommandResponse},
    connection::sqlite_connection::get_sqlite_handle,
    model::{model_common::ModelCommon, category::Category},
};

#[tauri::command]
pub async fn cmd_create_category(
    new_category_name: String,
    color_code_hex: String,
) -> Result<SqliteCommandResponse<usize>, InvokeError> {
    if !Category::is_valid_hex(&color_code_hex) {
        return Ok(SqliteCommandResponse::to_command_response_with_message(
            Category::new(new_category_name, color_code_hex).create(&get_sqlite_handle()),
            "Color hex code was invalid".to_owned(),
        ));
    }

    Ok(SqliteCommandResponse::to_command_response(
        Category::new(new_category_name, color_code_hex).create(&get_sqlite_handle()),
    ))
}

#[tauri::command]
pub async fn cmd_read_category(
    category_name: String,
) -> Result<SqliteCommandResponse<Option<Category>>, InvokeError> {
    Ok(Option::<Category>::to_command_response(
        Category::read(&category_name, &get_sqlite_handle()),
    ))
}

#[tauri::command]
pub async fn cmd_delete_category(
    category: String,
) -> Result<SqliteCommandResponse<usize>, InvokeError> {
    Ok(SqliteCommandResponse::to_command_response(
        Category::delete(&category, &get_sqlite_handle()),
    ))
}

#[tauri::command]
pub async fn cmd_read_list_category(
) -> Result<SqliteCommandResponse<Vec<Category>>, InvokeError> {
    Ok(Vec::<Category>::to_command_response(
        Category::read_list(&get_sqlite_handle()),
    ))
}


#[tauri::command]
pub async fn cmd_category_toggle_visibility(
    category: String,
) -> Result<SqliteCommandResponse<usize>, InvokeError> {
    Ok(SqliteCommandResponse::to_command_response(
        Category::update_category_toggle_visisbility(&category, &get_sqlite_handle()),
    ))
}
