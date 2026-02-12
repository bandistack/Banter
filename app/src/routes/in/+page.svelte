<script lang="ts">
  import { goto } from "$app/navigation";

async function intw(): Promise<void> {
  console.log("hola")
  const params = new URLSearchParams({
    response_type: "code",
    client_id: "5nyrbmto67f2ni9srvookx5nen63pj",
    redirect_uri: "http://localhost:1420/cback",
    scope: "user:read:email chat:read chat:edit",
  });
  window.location.href = `https://id.twitch.tv/oauth2/authorize?${params.toString()}`;
}
</script>
<div class="flex flex-col h-screen w-screen justify-center items-center p-1 bg-slate-50 text-slate-900 dark:bg-slate-900 dark:text-slate-50">
  <h1 class="text-3xl font-bold mb-8">Log In</h1>
  <!-- Contenedor en fila -->
  <div class="flex items-center gap-3">
    <button
      on:click={intw}
      class="flex items-center gap-3 px-4 py-2 bg-purple-800 text-slate-50 rounded-sm hover:bg-purple-700 active:bg-purple-950 text-sm sm:text-base md:text-lg">
      <span class="leading-none font-normal">Log in with Twitch</span>
    </button>
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 2400 2800"
        class="w-6 h-6 fill-current bg-slate-50 text-slate-900 dark:bg-slate-900 dark:text-slate-50">
      <path d="M0 0v2200h600v600l600-600h600l600-600V0H0zm2100 1200l-300 300h-600l-300 300v-300H600V300h1500v900z"/>
      <path d="M1800 600h-300v600h300V600zm-600 0h-300v600h300V600z"/>
    </svg>
  </div>
</div>