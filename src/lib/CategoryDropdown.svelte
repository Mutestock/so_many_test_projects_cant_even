<script>
    import { onMount } from "svelte";
    import { currentlySelectedCategory } from "$lib/stores/creationStore";
    import { CategoryInvocation } from "$lib/invocations/categoryInvocations";

    let selectedCategory;
    let allCategories = [];

    $: currentlySelectedCategory.set(selectedCategory);

    onMount(async () => {
        allCategories = await CategoryInvocation.readListCategory();
    });
</script>

<select bind:value={selectedCategory}>
    {#each allCategories as category}
        <option value={category.name}>{category.name}</option>
    {/each}
</select>
