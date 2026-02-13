<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { goto } from "$app/navigation";

  onMount(async () => {
    const hashParams = new URLSearchParams(window.location.hash.substring(1));
    const actk = hashParams.get("access_token");
    const idtk = hashParams.get("id_token");
    const expectedNonce = sessionStorage.getItem("tw_oauth_nonce");

    try {
      if (idtk && expectedNonce) {
        const payload = JSON.parse(atob(idtk.split(".")[1]));
        if (payload.nonce !== expectedNonce) {
          throw new Error("Invalid OAuth nonce");
        }
      }
      await invoke("chtk", {
        actk,
        idtk
      });
      sessionStorage.removeItem("tw_oauth_state");
      sessionStorage.removeItem("tw_oauth_nonce");
      goto("/chat");
    } catch (err) {
      console.error("Error al validar token:", err);
      goto("/in");
    }
  });
</script>
<div class="flex h-screen w-screen items-center justify-center bg-slate-50 dark:bg-slate-900 text-slate-900 dark:text-slate-50">
    <div class="w-12 h-12 border-4 border-slate-900 dark:border-slate-50 border-t-transparent dark:border-t-transparent rounded-full animate-spin"></div>
</div>
