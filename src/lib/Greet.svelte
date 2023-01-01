<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

  let sqlitePingRes = false;
  let sqliteSimpleInsertSucceeded = false;
  let sqliteSimpleRes = "";

  // Svelte isn't using typescript with Tauri although it is configured for it.
  // My linter, however, believes that I do use typescript
  let sqliteCategories = "[]";
  let oneCategory = "";

  async function pingSqlite() {
    sqlitePingRes = await invoke("cmd_sqlite_ping", {});
  }

  async function sqliteInsertSimple() {
    sqliteSimpleInsertSucceeded = await invoke("cmd_create_node", {
      nodeCategory: "event",
      name: "cake",
    });
  }

  async function sqliteGetCategories() {
    sqliteCategories = await invoke("cmd_read_list_node_category", {});
    console.log(sqliteCategories);
    oneCategory = JSON.stringify(sqliteCategories);
  }

  async function sqliteReadSimple() {
    sqliteSimpleRes = await invoke("cmd_read_node", {
      name: "cake",
    });
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
    <button on:click={sqliteInsertSimple}>Sqlite Insert</button>
    <p>Success: {sqliteSimpleInsertSucceeded}</p>
  </div>
  <div class="row">
    <button on:click={sqliteReadSimple}>Sqlite Read</button>
    <p>Get output: {sqliteSimpleRes}</p>
  </div>
  <div class="row">
    <p>misc:</p>
    <p>Sqlite ping res: {sqlitePingRes}</p>
  </div>
  <div class="row">
    <p>Sqlite all categories:</p>
    <p>{sqliteCategories}</p>
    <p>{oneCategory}</p>
  </div>
</div>
