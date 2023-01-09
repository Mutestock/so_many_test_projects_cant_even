<script>
    import { allNodesWithCategoriesTurnedOn } from "$lib/stores/overviewStore";
    import NavBar from "$lib/NavBar.svelte";
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

<NavBar />
<CategoryToggle/>

<div>
    <p>Nodes:</p>
    <table>
        <tr>
            <th>Name</th>
            <th>Date Added</th>
            <th>Date Modified</th>
            <th>Node Category</th>
        </tr>
        {#each _allNodesWithCategoriesTurnedOn as { name, date_added, date_modified, primary_image_path, category }}
            <tr>
                <td>{name}</td>
                <td>{date_added}</td>
                <td>{date_modified}</td>
                <td>{primary_image_path}</td>
                <td>{category}</td>
            </tr>
        {/each}
    </table>
</div>
