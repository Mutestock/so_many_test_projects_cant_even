<script>
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import NavBar from "$lib/NavBar.svelte";


let allNodes = [];


async function readAllNodes() {
    allNodes = await invoke("cmd_read_list_node",{});
}


onMount(async () => {
    await readAllNodes();
});


</script>

<NavBar/>

<div>
    <p>Nodes:</p>
    <table>
        <tr>
            <th>Name</th>
            <th>Date Added</th>
            <th>Date Modified</th>
            <th>Node Category</th>
        </tr>
        {#each allNodes as {name, date_added, date_modified, primary_image_path, node_category} }
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
