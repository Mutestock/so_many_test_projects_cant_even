import { invoke } from "@tauri-apps/api"

export async function readAllNodesWithToggledOnCategories() {
    let response = await invoke("cmd_read_list_toggled_on",{})
    return response.payload
}
