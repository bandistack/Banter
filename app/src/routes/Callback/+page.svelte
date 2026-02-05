<script lang="ts">
  import { page } from '$app/stores';
  import { onMount } from 'svelte';

  let code: string | null = null;

  onMount(() => {
    const urlParams = new URLSearchParams(window.location.search);
    code = urlParams.get("code");

    if (code) {
      // Aquí llamas a tu backend (Rust/Tauri) para intercambiar el code
      fetch("/api/exchange-token", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ code })
      })
      .then(res => res.json())
      .then(data => {
        console.log("Access token recibido:", data);
        // Aquí puedes guardar el token en un store o redirigir al chat
      })
      .catch(err => console.error("Error al intercambiar token:", err));
    }
  });
</script>

<div class="flex items-center justify-center h-screen text-white bg-gray-900">
  {#if code}
    <p>Procesando login con Twitch...</p>
  {:else}
    <p>No se recibió ningún código de autorización.</p>
  {/if}
</div>
