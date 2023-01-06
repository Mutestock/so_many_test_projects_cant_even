import { invoke } from "@tauri-apps/api"

// Again, currently typescript type checking just isn't clicking with svelte for some reason.

export async function readAllNodeCategories() {
    let response = await invoke("cmd_read_list_node_category",{})
    return response.payload
}