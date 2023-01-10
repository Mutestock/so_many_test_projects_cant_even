import { invoke } from "@tauri-apps/api"

// Again, currently typescript type checking just isn't clicking with svelte for some reason.

export async function readAllCategories() {
    let response = await invoke("cmd_read_list_category",{})
    return response.payload
}

export async function toggleCategory(category) {
    let response = await invoke("cmd_category_toggle_visibility", {
        category: category,
    });
    return response.payload
}