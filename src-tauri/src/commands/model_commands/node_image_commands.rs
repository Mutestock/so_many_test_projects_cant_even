use tauri::InvokeError;



#[tauri::command]
pub async fn cmd_create_node_image() -> Result<bool, InvokeError> {
    Ok(true)
}

#[tauri::command]
pub async fn cmd_read_node_image() -> Result<bool, InvokeError> {
    Ok(true)
}

#[tauri::command]
pub async fn cmd_update_node_image() -> Result<bool, InvokeError> {
    Ok(true)
}

#[tauri::command]
pub async fn cmd_delete_node_image() -> Result<bool, InvokeError> {
    Ok(true)
}

#[tauri::command]
pub async fn cmd_read_list_node_image() -> Result<bool, InvokeError> {
    Ok(true)
}