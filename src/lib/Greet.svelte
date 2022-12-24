<script>
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

  let rocksPingRes = false;
  let sqlitePingRes = false;
  let rocksGetKeyInput = "";
  let rocksPutKeyInput = "";
  let rocksPutValueInput = "";
  let rocksGetOutput = "";
  let rocksSetOutput = "";

  async function pingRocks() {
    rocksPingRes = await invoke("ping", {});
  }

  async function pingSqlite() {
    sqlitePingRes = await invoke("cmd_sqlite_ping", {})
  }

  async function rocksPut() {
    rocksSetOutput = await invoke("cmd_rocks_put", {
      key: rocksPutKeyInput,
      value: rocksPutValueInput,
    });
  }

  async function rocksGet() {
    rocksGetOutput = await invoke("cmd_rocks_get", {
      key: rocksGetKeyInput,
    });
  }

  onMount(async () => {
    await pingRocks();
    await pingSqlite();
  });
 
</script>

<div>
  <div class="row">
    <button on:click={pingRocks}> Do the ping</button>
  </div>
  <div class="row">
    <button on:click={rocksPut}>Rocks put</button>
    <input
      id="rocks-put-key-input"
      bind:value={rocksPutKeyInput}
      placeholder="Rocks Put Key"
    />
    <input
      id="rocks-put-value-input"
      bind:value={rocksPutValueInput}
      placeholder="Rocks Put Value"
    />
    <p>Put output: {rocksSetOutput}</p>
  </div>
  <div class="row">
    <button on:click={rocksGet}>Rocks get</button>
    <input
      id="rocks-get-key-input"
      bind:value={rocksGetKeyInput}
      placeholder="Rocks get key"
    />
    <p>Get output: {rocksGetOutput}</p>
  </div>
  <div class="row">
    <p>misc: </p>
    <p>Rocks ping res: {rocksPingRes}</p>
    <p>Sqlite ping res: {sqlitePingRes}</p>
  </div>
</div>
