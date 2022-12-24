use crate::{
    model::node::{Node, NodeComment},
};
use tauri::{AppHandle, InvokeError};

#[tauri::command]
pub async fn cmd_new_node(
    handle: AppHandle,
    node_type: usize,
    custom_node_type_name: Option<String>,
    name: String,
) -> Result<bool, InvokeError> {
    Ok(rocks_put_on_non_duplicate(
        &handle,
        &name,
        Node::new(node_type, name.clone(), custom_node_type_name)
            .expect("Node creation failed")
            .as_json()
            .expect("Could not serialize Node")
            .as_str(),
    )
    .expect("Rocks failed to put on non-duplicate"))
}

#[tauri::command]
pub async fn cmd_append_comment_to_node(
    handle: AppHandle,
    comment_content: String,
    node_name: String,
) -> Result<bool, InvokeError> {
    Ok({
        let rocks_get_res = rocks_get(&handle, &node_name).unwrap();
        if rocks_get_res.is_empty() {
            false
        } else {
            let node: Node = serde_json::from_str(rocks_get_res.as_str()).unwrap();
            let node_string = node
                .and_add_comment(NodeComment::new(comment_content))
                .as_json()
                .expect("Could not serialize Node");
            rocks_put(&handle, &node_name, &node_string).expect("Rocks failed to append comment to node")
        }
    })
}
