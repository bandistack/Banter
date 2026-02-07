<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { goto } from '$app/navigation';
  import { type UserToken } from "$lib/usertoken";
  import { UserTokenPersistence } from "$lib/tokenpersistence";

  onMount(async () => {
    const code = new URLSearchParams(window.location.search).get("code");
    if (code) {
      try {
        const token: UserToken = await invoke("exchange_token", { code });
        if (token?.access_token) {
          UserTokenPersistence.save(token);
          goto("/Chat");
        } else {
          goto("/Login");
        }
      } catch (err) {
        console.error("Error al intercambiar token:", err);
      }
    }
  });
</script>
<div class="flex h-screen w-screen items-center justify-center bg-gray-900 text-slate-50">
    <div class="w-12 h-12 border-4 border-purple-500 border-t-transparent rounded-full animate-spin"></div>
</div>
