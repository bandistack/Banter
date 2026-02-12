<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { goto } from '$app/navigation';
  import { type UserToken } from "$lib/usertoken";
  import { UserTokenPersistence } from "$lib/tokenpersistence";

onMount(async () => {
  const code = new URLSearchParams(window.location.search).get("code");
  console.log("hola");
  if (code) {
    try {
      const token: UserToken = await invoke("exchange_token", { code });
      UserTokenPersistence.save(token);
      await invoke("get_current_user");
      goto("/Chat");
    } catch (err) {
      console.error("Error al validar token:", err);
      UserTokenPersistence.clear();
      goto("/Login");
      return;
    }
  }
  const existing = UserTokenPersistence.load();
  if (existing) {
    try {
      await invoke("get_current_user"); // valida token en backend
      goto("/Chat");
      return;
    } catch (err) {
      console.error("Token inválido o expirado:", err);
      UserTokenPersistence.clear();
    }
  }
  goto("/Login");
});
</script>
<div class="flex h-screen w-screen items-center justify-center bg-slate-50 dark:bg-slate-900 text-slate-900 dark:text-slate-50">
    <div class="w-12 h-12 border-4 border-slate-900 dark:border-slate-50 border-t-transparent dark:border-t-transparent rounded-full animate-spin"></div>
</div>
