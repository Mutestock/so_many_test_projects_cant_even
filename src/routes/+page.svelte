<script>
  import { menuEventListenerInit } from "../lib/tauriMenu";
  import Home from "$lib/Home.svelte";
  import NavBar from "$lib/NavBar.svelte";

  menuEventListenerInit().then((x) => x);
</script>

<div>
  <NavBar />
  <div class="row">
    <Home />
  </div>
</div>

<style>
</style>
