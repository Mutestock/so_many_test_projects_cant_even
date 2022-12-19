<script>
  import { invoke } from "@tauri-apps/api/tauri";

  let name = "";
  let greetMsg = "";
  let pingRes = false;

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("greet", { name });
  }

  async function pingRocks(){
    pingRes = await invoke("ping", {});
  }
</script>

<div>
  <div class="row">
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button on:click={greet}> Greet </button>
    <button on:click={pingRocks}> Do the ping</button>
  </div>
  <p>{greetMsg}</p>
  <p>{pingRes}</p>
</div>
