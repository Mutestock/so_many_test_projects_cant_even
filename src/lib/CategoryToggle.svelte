<script>
    import { onMount } from "svelte";
    import { cloneDeep } from "lodash";
    import { allNodesWithCategoriesTurnedOn } from "./stores/overviewStore";
    import { CategoryInvocation } from "./invocations/categoryInvocations";
    import { NodeInvocation } from "./invocations/nodeInvocations";

    let _allNodesWithCategoriesTurnedOn = [];
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
    }

    onMount(async () => {
        allCategories = await CategoryInvocation.readListCategory();
        allCategoriesBuffer = cloneDeep(allCategories);
        refreshNodes();
    });
</script>

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

<style lang="scss">

    .category-input {
        color: var(--color, black);
    }
</style>
