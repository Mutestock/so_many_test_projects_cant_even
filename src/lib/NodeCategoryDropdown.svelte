<script>
    import { onMount } from "svelte";
    import { currentlySelectedCategory } from "$lib/stores/overviewStore";
    import { readAllNodeCategories } from "$lib/invocations/nodeCategoryInvocations";

    let selectedCategory;
    $: currentlySelectedCategory.set(selectedCategory);

    let allCategories = [];

    onMount(async () => {
        allCategories = await readAllNodeCategories();
    });
</script>

<select bind:value={selectedCategory}>
    {#each allCategories as category}
        <option value={category.name}>{category.name}</option>
    {/each}
</select>
