<script>
    import { onMount } from "svelte";
    import { currentlySelectedCategory } from "$lib/stores/creationStore";
    import { invokeReadAllCategories } from "$lib/invocations/categoryInvocations";
    let selectedCategory;
    $: currentlySelectedCategory.set(selectedCategory);
    let allCategories = [];
    onMount(async () => {
        allCategories = await invokeReadAllCategories();
    });
</script>

<select bind:value={selectedCategory}>
    {#each allCategories as category}
        <option value={category.name}>{category.name}</option>
    {/each}
</select>
