<script>
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import NavBar from "$lib/NavBar.svelte";
    import NodeCategoryDropdown from "$lib/NodeCategoryDropdown.svelte";

    let visibleNodes = [];
    let currentSelectedCategory = "";

    async function readAllNodes() {
        allNodes = await invoke("cmd_read_list_node", {});
    }

    async function readNodesByCategory() {
        allNodes = await invoke("cmd_read_nodes_by_node_category", {
            nodeCategory: currentCategory,
        });
    }

    onMount(async () => {
        await readAllNodes();
    });
</script>

<NavBar />

<div>
    <div>
        <p>Selected Category = {currentSelectedCategory}</p> 
        <NodeCategoryDropdown selected={currentSelectedCategory} />
    </div>
    <p>Nodes:</p>
    <table>
        <tr>
            <th>Name</th>
            <th>Date Added</th>
            <th>Date Modified</th>
            <th>Node Category</th>
        </tr>
        {#each visibleNodes as { name, date_added, date_modified, primary_image_path, node_category }}
            <tr>
                <td>{name}</td>
                <td>{date_added}</td>
                <td>{date_modified}</td>
                <td>{primary_image_path}</td>
                <td>{node_category}</td>
            </tr>
        {/each}
    </table>
</div>
