<script>
    import { onMount } from "svelte";
    import { cloneDeep } from "lodash";
    import { allNodesWithCategoriesTurnedOn } from "./stores/overviewStore";
    import { CategoryInvocation } from "./invocations/categoryInvocations";
    import { NodeInvocation } from "./invocations/nodeInvocations";
    import { NodeVisualInvocation } from "./invocations/nodeVisualInvocations";

    let _allNodesWithCategoriesTurnedOn = [];
    let nodeVisuals = [];
    let allCategories = [];
    let allCategoriesBuffer = [];

    // Should still consider having state handle more of these interactions.
    // Using the database is expensive.
    $: {
        allCategories.every((category, index) => {
            if (
                allCategoriesBuffer[index].visibilityToggledOn !=
                category.visibilityToggledOn
            ) {
                CategoryInvocation.categoryToggleVisibility(category.name).then(
                    () => refreshNodes()
                );
                allCategoriesBuffer[index].visibilityToggledOn =
                    category.visibilityToggledOn;
                return false;
            }
            return true;
        });
    }

    $: allNodesWithCategoriesTurnedOn.set(_allNodesWithCategoriesTurnedOn);

    async function refreshNodes() {
        _allNodesWithCategoriesTurnedOn =
            await NodeInvocation.readNodeListToggledOn();

        nodeVisuals = await NodeVisualInvocation.readNodeVisualList();
    }

    onMount(async () => {
        allCategories = await CategoryInvocation.readListCategory();
        allCategoriesBuffer = cloneDeep(allCategories);
        refreshNodes();
    });
</script>

<div class="toggle-container-outer ">
    <div class="toggle-container">
        {#each allCategories as category}
            <label
                class="category-input"
                style="--color:{category.colorCodeHex}"
            >
                <input
                    type="checkbox"
                    bind:checked={category.visibilityToggledOn}
                />
                {category.name}
            </label>
        {/each}
    </div>
</div>

<style lang="scss">
    $sidebar-background-color: #0f0f0f98;
    .category-input {
        color: var(--color, black);
    }
    .toggle-container-outer {
        top: 0;
        max-width: 10%;
        min-width: 200px;
        float: left;
        display: inline;
        background-color: $sidebar-background-color;
    }
    .toggle-container {
        width: 100%;
        align-self: stretch;
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: flex-start;
        float: left;
    }
</style>
