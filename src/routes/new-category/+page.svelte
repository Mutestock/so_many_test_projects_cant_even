<script>
    import { invokeCreateCategory } from "$lib/invocations/categoryInvocations";
    import { invokeWriteLog, LogLevel } from "$lib/log";
    import { HsvPicker } from "svelte-color-picker";

    let categoryName = "";
    let hexColor = "";

    async function newCategory() {
        await invokeCreateCategory(categoryName, hexColor);
        await invokeWriteLog(LogLevel.Info, `New category created: ${categoryName}`)
    }

    function colorCallback(rgba) {
        hexColor = rgbToHex(rgba.detail)
        console.log(hexColor);
    }

    function rgbToHex(rgba) {
        return (
            "#" + ((1 << 24) + (rgba.r << 16) + (rgba.g << 8) + rgba.b).toString(16).slice(1)
        );
    }
</script>

<div>
    <p>This is where you create categories</p>
    <div class="row">
        <div>
            <p>Category name</p>
            <input bind:value={categoryName} />
        </div>
        <div>
            <p>Hex color</p>
            <HsvPicker on:colorChange={colorCallback} startColor={"#FBFBFB"} />
        </div>
    </div>
    <button on:click={newCategory}> Create Category </button>
</div>
