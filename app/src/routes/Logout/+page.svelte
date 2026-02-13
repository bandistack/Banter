<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { goto } from "$app/navigation";

  let loading = false;

  async function handleLogout() {
    loading = true;
    try {
      await invoke("twitch_disconnect");
      await invoke("logout");
      goto("/");
    } catch (err) {
      console.error("Error en logout:", err);
      loading = false;
    }
  }
</script>

<div class="flex px-4 py-1 mt-auto mb-2 hover:bg-slate-200 dark:hover:bg-slate-800 active:bg-slate-100 active:dark:bg-slate-900">
  {#if loading}
    <span>Saliendo…</span>
  {:else}
    <button on:click={handleLogout}>
      <span>Log Out</span>
    </button>
  {/if}
</div>