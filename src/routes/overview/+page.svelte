<script>
    import { allNodesWithCategoriesTurnedOn } from "$lib/stores/overviewStore";
    import CategoryToggle from "$lib/CategoryToggle.svelte";
    import { onDestroy } from "svelte";
    import OverviewSidePanel from "$lib/OverviewSidePanel.svelte";

    let _allNodesWithCategoriesTurnedOn;

    const unsubscribe = allNodesWithCategoriesTurnedOn.subscribe((value) => {
        _allNodesWithCategoriesTurnedOn = value;
    });

    onDestroy(async () => {
        unsubscribe();
    });
</script>

<CategoryToggle />
<div class="content-container">
    <p>Nodes:</p>
    <table>
        <tr>
            <th>Name</th>
            <th>Date Added</th>
            <th>Date Modified</th>
            <th>Node Category</th>
        </tr>
        {#each _allNodesWithCategoriesTurnedOn as { name, dateAdded, dateModified, primaryImagePath, category }}
            <tr>
                <td>{name}</td>
                <td>{dateAdded}</td>
                <td>{dateModified}</td>
                <td>{primaryImagePath}</td>
                <td>{category}</td>
            </tr>
        {/each}
    </table>
</div>
<OverviewSidePanel />

<style lang="scss">
    .content-container {
        display: inline;
        float: left;
        align-self: auto;
        justify-content: center;
    }
</style>
