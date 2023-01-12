<script>
    import { allNodesWithCategoriesTurnedOn } from "$lib/stores/overviewStore";
    import CategoryToggle from "$lib/CategoryToggle.svelte";
    import { onDestroy } from "svelte";
    import OverviewSidePanel from "$lib/OverviewSidePanel.svelte";
    import { spring } from "svelte/motion";

    let _allNodesWithCategoriesTurnedOn;

    const unsubscribe = allNodesWithCategoriesTurnedOn.subscribe((value) => {
        _allNodesWithCategoriesTurnedOn = value;
    });

    let mouseIsDown = false;

    let coords = spring(
        { x: 5, y: 5 },
        {
            stiffness: 0.12,
            damping: 0.62,
        }
    );

    let size = spring(10);

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

<svg
    on:mousemove={(e) => {
        if ((mouseIsDown == true)) {
            console.log(mouseIsDown);
            
            coords.set({ x: e.clientX, y: e.clientY });
        }
    }}
    on:mousedown={() => {
        size.set(20);
        mouseIsDown = true;
    }}
    on:mouseup={() => {
        size.set(10);
        mouseIsDown = false;
    }}
>
    <circle cx={$coords.x} cy={$coords.y} r={$size} />
</svg>

<OverviewSidePanel />

<style lang="scss">
    .content-container {
        display: inline;
        float: left;
        align-self: auto;
        justify-content: center;
    }

    svg {
        position: absolute;
        width: 100%;
        height: 100%;
        left: 0;
        top: 0;
    }
</style>
