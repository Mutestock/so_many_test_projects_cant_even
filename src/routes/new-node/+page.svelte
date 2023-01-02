<script>
    import NavBar from "$lib/NavBar.svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    let nodeName = "";
    let nodeCategory = "";

    async function newNode() {
        invoke ("cmd_create_node",{
            "nodeCategory" : nodeCategory,
            "name" : nodeName
        });
    }

</script>

<div>
    <NavBar />
    <p>Hello there. This is the new node tab</p>
    <div class="row">
        <p>Name</p>
        <input bind:value={nodeName} />
    </div>

    <div class="row">
        <p>Node Category</p>
        <input bind:value={nodeCategory} />
    </div>
    <button on:click={newNode}>
        Create Node
    </button>
</div>
