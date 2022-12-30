use tauri::InvokeError;



#[tauri::command]
pub async fn cmd_create_node_category() -> Result<bool, InvokeError> {
    Ok(true)
}

#[tauri::command]
pub async fn cmd_read_node_category() -> Result<bool, InvokeError> {
    Ok(true)
}

#[tauri::command]
pub async fn cmd_update_node_category() -> Result<bool, InvokeError> {
    Ok(true)
}

#[tauri::command]
pub async fn cmd_delete_node_category() -> Result<bool, InvokeError> {
    Ok(true)
}

#[tauri::command]
pub async fn cmd_read_list_node_category() -> Result<bool, InvokeError> {
    Ok(true)
}