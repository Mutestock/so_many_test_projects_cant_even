<script>
    import NavBar from "$lib/NavBar.svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import { open } from "@tauri-apps/api/dialog";
    import { appDataDir } from "@tauri-apps/api/path";
    let nodeName = "";
    let nodeCategory = "";

    async function newNode() {
        invoke ("cmd_create_node",{
            "nodeCategory" : nodeCategory,
            "name" : nodeName
        });
    }

    async function appendNodeImage() {
        let selected = await open({
            directory: false,
            multiple: false,
            defaultPath: await appDataDir()
        });
        console.log(selected);
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
    <button on:click={appendNodeImage}>Select Image</button>
</div>
