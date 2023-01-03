<script>
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import { currentlySelectedCategory } from "./stores/overviewStore";

    let selectedCategory;
    $: currentlySelectedCategory.set(selectedCategory);

    let allCategories = [];

    async function readAllNodeCategories() {
        allCategories = await invoke("cmd_read_list_node_category",{})
    }

    onMount(async () => {
        await readAllNodeCategories();
    });
</script>

<select bind:value={selectedCategory}>
    {#each allCategories as category}
        <option value={category.name}>{category.name}</option>
    {/each}
</select>
