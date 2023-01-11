<script>
    import { allNodesWithCategoriesTurnedOn } from "$lib/stores/overviewStore";
    import CategoryToggle from "$lib/CategoryToggle.svelte";
    import { onDestroy } from "svelte";

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



<style lang="scss">
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
        width: 25%;
        float: left;
        display: inline;
    }

    .content-container {
        width: 50%;
        margin: 0 auto;
        display: inline;
        float: left;
    }
</style>
