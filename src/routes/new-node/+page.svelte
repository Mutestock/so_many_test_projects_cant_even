<script>
    import NavBar from "$lib/NavBar.svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import { open } from "@tauri-apps/api/dialog";
    import { appDataDir } from "@tauri-apps/api/path";
    import { writeLog, LogLevel } from "$lib/log";
    let nodeName = "";
    let nodeCategory = "";
    let image_path = "";
    let image_appended = false;

    async function newNode() {
        invoke ("cmd_create_node",{
            "nodeCategory" : nodeCategory,
            "name" : nodeName
        });
        if (image_appended) {
            invoke ("cmd_create_node_image",{
                imageTitle: `${nodeName}PrimaryImage`,
                nodeName: nodeName
            })
        }
        await writeLog(LogLevel.Info, `New node created: ${nodeName} - Type: ${nodeCategory}`);
        nodeName = "";
        nodeCategory ="";
        image_appended = false;
        image_path = "";
    }

    async function appendNodeImage() {
        let selectedPath = await open({
            directory: false,
            multiple: false,
            defaultPath: await appDataDir()
        });
        if (!selectedPath) {
            writeLog(LogLevel.Info, "Attempted node image append was empty");
        }
        else {
            image_path = selectedPath;
            image_appended = true;
        }
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
