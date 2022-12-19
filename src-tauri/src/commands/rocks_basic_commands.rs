use tauri::{InvokeError, AppHandle};
use crate::connection::rocks_connection::connect;



#[tauri::command]
pub async fn ping(handle: AppHandle) -> Result<bool, InvokeError>{
    Ok(match connect(handle){
        Ok(_) => true,
        Err(_) => false
    })
}