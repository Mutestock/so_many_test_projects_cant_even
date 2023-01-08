import { invoke } from "@tauri-apps/api"

// Again, currently typescript type checking just isn't clicking with svelte for some reason.

export async function readAllNodeCategories() {
    let response = await invoke("cmd_read_list_node_category",{})
    return response.payload
}

export async function toggleNodeCategory(nodeCategory) {
    let response = await invoke("cmd_category_toggle_visibility", {
        nodeCategory: nodeCategory,
    });
    return response.payload
}