<script>
    import { onMount } from "svelte";
    import {
        readAllNodeCategories,
        toggleNodeCategory,
    } from "./invocations/nodeCategoryInvocations";
    import { readAllNodesWithToggledOnCategories } from "./invocations/nodeInvocations";
    import { cloneDeep } from "lodash";

    let allNodesWithCategoriesTurnedOn = [];
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
                toggleNodeCategory(category.name)
                    .then(()=>refreshNodes());
                allCategoriesBuffer[index].visibility_toggled_on =
                    category.visibility_toggled_on;
                return false;
            }
            return true;
        });
    }


    async function refreshNodes() {
        allNodesWithCategoriesTurnedOn =
            await readAllNodesWithToggledOnCategories();
    }

    onMount(async () => {
        allCategories = await readAllNodeCategories();
        allCategoriesBuffer = cloneDeep(allCategories);
        allNodesWithCategoriesTurnedOn =
            await readAllNodesWithToggledOnCategories();
    });
</script>

{#each allCategories as category}
    <label>
        <input type="checkbox" bind:checked={category.visibility_toggled_on} />
        {category.name}
    </label>
{/each}
<table>
    <tr>
        <th>Name</th>
        <th>Date Added</th>
        <th>Date Modified</th>
        <th>Node Category</th>
    </tr>
    {#each allNodesWithCategoriesTurnedOn as { name, date_added, date_modified, primary_image_path, node_category }}
        <tr>
            <td>{name}</td>
            <td>{date_added}</td>
            <td>{date_modified}</td>
            <td>{primary_image_path}</td>
            <td>{node_category}</td>
        </tr>
    {/each}
</table>
