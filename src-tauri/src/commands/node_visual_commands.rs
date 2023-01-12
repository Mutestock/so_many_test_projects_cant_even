use tauri::InvokeError;

use crate::{
    connection::sqlite_connection::get_sqlite_handle, model::hybrids::node_visual::NodeVisual,
};

use super::command_utils::{CommandResponseComposable, SqliteCommandResponse};

#[tauri::command]
pub fn cmd_read_list_node_visual(
) -> Result<SqliteCommandResponse<Vec<NodeVisual>>, InvokeError> {
    Ok(Vec::<NodeVisual>::to_command_response(
        NodeVisual::read_list(&get_sqlite_handle()),
    ))
}
