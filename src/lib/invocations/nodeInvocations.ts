import { invoke } from "@tauri-apps/api"

export async function invokeReadAllNodesWithToggledOnCategories() {
    let response = await invoke("cmd_read_list_toggled_on",{})
    return response.payload
}
