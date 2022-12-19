<script>
  import { invoke } from "@tauri-apps/api/tauri";

  let name = "";
  let greetMsg = "";
  let pingRes = false;
  let rocksGetKeyInput = "";
  let rocksPutKeyInput = "";
  let rocksPutValueInput = "";
  let rocksGetOutput = "";
  let rocksSetOutput = "";

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("greet", { name });
  }

  async function pingRocks() {
    pingRes = await invoke("ping", {});
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
    <p>misc:</p>
    <p>Ping res: {pingRes}</p>
  </div>
</div>
