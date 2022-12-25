<script>
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

  let rocksPingRes = false;
  let sqlitePingRes = false;
  let sqliteSimpleInsertSucceeded = false;
  let sqliteSimpleRes = "";

  async function pingSqlite() {
    sqlitePingRes = await invoke("cmd_sqlite_ping", {});
  }

  async function sqliteInsertSimple() {
    sqliteSimpleInsertSucceeded = await invoke("cmd_new_node",
      {
        "nodeCategory": "event",
        "name": "cake"
      }
    );
  }

  async function sqliteReadSimple() {
    sqliteSimpleRes = await invoke("cmd_sqlite_read", {
      "name": "cake"
    }).then(x=>x.json());
  }

  onMount(async () => {
    await pingSqlite();
  });
 
</script>

<div>
  <div class="row">
    <button on:click={pingSqlite}> Do the ping</button>
  </div>
  <div class="row">
    <button on:click={sqliteInsertSimple}>Sqlite Insert</button>
    <p>Success: {sqliteSimpleInsertSucceeded}</p>
  </div>
  <div class="row">
    <button on:click={sqliteReadSimple}>Sqlite Read</button>
    <p>Get output: {sqliteSimpleRes}</p>
  </div>
  <div class="row">
    <p>misc: </p>
    <p>Sqlite ping res: {sqlitePingRes}</p>
  </div>
</div>
