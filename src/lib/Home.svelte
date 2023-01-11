<script>
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

  let sqlitePingRes = false;
  let oneCategory = "";

  async function pingSqlite() {
    sqlitePingRes = await invoke("cmd_sqlite_ping", {});
  }

  async function sqliteGetCategories() {
    let response = await invoke("cmd_read_list_category", {});
    oneCategory = JSON.stringify(response.payload);
  }

  onMount(async () => {
    await pingSqlite();
    await sqliteGetCategories();
  });
</script>

<div>
  <div class="row">
    <button on:click={pingSqlite}> Do the ping</button>
  </div>
  <div class="row">
    <p>Sqlite ping res: {sqlitePingRes}</p>
  </div>
</div>
