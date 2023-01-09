<script>
    import { onMount } from "svelte";
    import {
        readAllNodeCategories,
        toggleCategory,
    } from "./invocations/categoryInvocations";
    import { readAllNodesWithToggledOnCategories } from "./invocations/nodeInvocations";
    import { cloneDeep } from "lodash";
    import { allNodesWithCategoriesTurnedOn } from "./stores/overviewStore";

    let _allNodesWithCategoriesTurnedOn = [];
    let allCategories = [];
    let allCategoriesBuffer = [];

    // Should still consider having state handle more of these interactions.
    // Using the database is expensive.
    $: {
        allCategories.every((category, index) => {
            if (
                allCategoriesBuffer[index].visibility_toggled_on !=
                category.visibility_toggled_on
            ) {
                toggleCategory(category.name).then(() => refreshNodes());
                allCategoriesBuffer[index].visibility_toggled_on =
                    category.visibility_toggled_on;
                return false;
            }
            return true;
        });
    }

    $: allNodesWithCategoriesTurnedOn.set(_allNodesWithCategoriesTurnedOn)

    async function refreshNodes() {
        _allNodesWithCategoriesTurnedOn =
            await readAllNodesWithToggledOnCategories();
    }

    onMount(async () => {
        allCategories = await readAllNodeCategories();
        allCategoriesBuffer = cloneDeep(allCategories);
        refreshNodes();
    });
</script>

{#each allCategories as category}
    <label>
        <input type="checkbox" bind:checked={category.visibility_toggled_on} />
        {category.name}
    </label>
{/each}