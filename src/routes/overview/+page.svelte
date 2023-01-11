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

<div class="toggle-container-outer ">
    <div class="toggle-container">
        <CategoryToggle />
    </div>
</div>
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
<div class="right-side-overview">
    <div>
        <OverviewSidePanel />
    </div>
</div>

<style lang=scss>
    $sidebar-background-color:  #0f0f0f98;

    .toggle-container {
        width: 100%;
        align-self: stretch;
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: flex-start;
        float: left;
    }

    .toggle-container-outer {
        top: 0;
        max-width: 10%;
        min-width: 200px;
        float: left;
        display: inline;
        background-color: $sidebar-background-color;
    }

    .content-container {
        display: inline;
        float: left;
        align-self: auto;
        justify-content: center;
    }

    .right-side-overview {
        position: fixed;
        right: 0;
        bottom: 0;
        top: 0;
        background: $sidebar-background-color;
        float: left;
        width: 20%
    }
</style>
