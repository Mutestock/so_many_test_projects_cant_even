<script>
  import { menuEventListenerInit } from "../lib/tauriMenu";
  import Home from "$lib/Home.svelte";
  import NavBar from "$lib/NavBar.svelte";

  menuEventListenerInit();
</script>

<div>
  <div class="row">
    <Home />
  </div>
</div>

<style>
</style>
