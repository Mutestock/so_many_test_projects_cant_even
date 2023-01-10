<script>
    import NavBar from "$lib/NavBar.svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import { open } from "@tauri-apps/api/dialog";
    import { appDataDir } from "@tauri-apps/api/path";
    import { writeLog, LogLevel } from "$lib/log";
    import CategoryDropdown from "$lib/CategoryDropdown.svelte";
    import { currentlySelectedCategory } from "$lib/stores/creationStore";
    import { onDestroy } from "svelte";
    import { copyFile, BaseDirectory } from '@tauri-apps/api/fs';

    let nodeName = "";
    let currentlySelectedCategoryValue;
    let image_path = "";
    let image_appended = false;

    async function newNode() {
        invoke("cmd_create_node", {
            category: currentlySelectedCategoryValue,
            name: nodeName,
        });
        if (image_appended) {
            invoke("cmd_create_image", {
                imageTitle: `${nodeName}_primary_image`,
                nodeName: nodeName,
            });
        }
        await writeLog(
            LogLevel.Info,
            `New node created: ${nodeName} - Type: ${currentlySelectedCategoryValue}`
        );
        nodeName = "";
        image_appended = false;
        image_path = "";
    }

    const unsubscribe = currentlySelectedCategory.subscribe((value) => {
        currentlySelectedCategoryValue = value;
    });

    async function appendImage() {
        let selectedPath = await open({
            directory: false,
            multiple: false,
            defaultPath: await appDataDir(),
        });
        if (!selectedPath) {
            writeLog(LogLevel.Info, "Attempted node image append was empty");
        } else {
            console.log(selectedPath);
            let appDataDirectory = await appDataDir();
            let imagesDirectory = appDataDirectory + "images";
            let file_extension = selectedPath.split(".").at(-1);
            await copyFile(selectedPath, `images/${nodeName}_primary_image.${file_extension}`, {dir: BaseDirectory.AppData});
            image_path = imagesDirectory + "/" + `${nodeName}_primary_image.${file_extension}`;
            image_appended = true;
        }
    }

    onDestroy(async () => {
        currentlySelectedCategory.set("");
        unsubscribe();
    });
</script>

<div>
    <NavBar />
    <p>Hello there. This is the new node tab</p>
    <div class="row">
        <p>Name</p>
        <input bind:value={nodeName} />
    </div>
    <img src="images/aaaarsc_primary_image.png" alt="">

    <CategoryDropdown />
    <button on:click={newNode}> Create Node </button>
    <button on:click={appendImage}>Select Image</button>
</div>
