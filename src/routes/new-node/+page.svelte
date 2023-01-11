<script>
    import NavBar from "$lib/NavBar.svelte";
    import { open } from "@tauri-apps/api/dialog";
    import { appDataDir } from "@tauri-apps/api/path";
    import { invokeWriteLog, LogLevel } from "$lib/log";
    import CategoryDropdown from "$lib/CategoryDropdown.svelte";
    import { currentlySelectedCategory } from "$lib/stores/creationStore";
    import { onDestroy } from "svelte";
    import { copyFile, BaseDirectory } from "@tauri-apps/api/fs";
    import { NodeInvocation } from "$lib/invocations/nodeInvocations";
    import { ImageInvocation } from "$lib/invocations/imageInvocations";

    let nodeName = "";
    let currentlySelectedCategoryValue;
    let imagePath = "";
    let imageAppended = false;

    async function newNode() {
        await NodeInvocation.createNode(
            currentlySelectedCategoryValue,
            nodeName
        );

        if (imageAppended) {
            let imageTitle = `${nodeName}_primary_image`;
            await ImageInvocation.createImage(imageTitle, nodeName);
        }
        await invokeWriteLog(
            LogLevel.Info,
            `New node created: ${nodeName} - Type: ${currentlySelectedCategoryValue}`
        );
        nodeName = "";
        imageAppended = false;
        imagePath = "";
    }

    const unsubscribe = currentlySelectedCategory.subscribe((value) => {
        currentlySelectedCategoryValue = value;
    });

    async function appendImage()  {
        let selectedPath = await open({
            directory: false,
            multiple: false,
            defaultPath: await appDataDir(),
        });
        if (!selectedPath) {
            await invokeWriteLog(
                LogLevel.Info,
                "Attempted node image append was empty"
            );
        } else {
            let appDataDirectory = await appDataDir();
            let imagesDirectory = appDataDirectory + "images";
            let fileExtension = selectedPath.split(".").at(-1);

            await copyFile(
                selectedPath,
                `images/${nodeName}_primary_image.${fileExtension}`,
                { dir: BaseDirectory.AppData }
            );

            imagePath =
                imagesDirectory +
                "/" +
                `${nodeName}_primary_image.${fileExtension}`;
            imageAppended = true;
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
    <img src="" alt="" />

    <CategoryDropdown />
    <button on:click={newNode}> Create Node </button>
    <button on:click={appendImage}>Select Image</button>
</div>
