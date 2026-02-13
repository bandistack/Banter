<script lang="ts">
  import { clid, reur } from "$lib/credential";

  function randomString(length: number): string {
    const charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    const values = new Uint8Array(length);
    crypto.getRandomValues(values);
    return Array.from(values).map((x) => charset[x % charset.length]).join("");
  }

  function intw(): void {
    const state = randomString(24);
    const nonce = randomString(24);
    sessionStorage.setItem("tw_oauth_state", state);
    sessionStorage.setItem("tw_oauth_nonce", nonce);

    const params = new URLSearchParams({
      client_id: $clid,
      redirect_uri: $reur,
      response_type: "token id_token",
      scope: "chat:read chat:edit openid",
      state,
      nonce,
    });
    window.location.href = `https://id.twitch.tv/oauth2/authorize?${params.toString()}`;
  }
</script>
<div class="flex flex-col h-screen w-screen justify-center items-center p-1 bg-slate-50 text-slate-900 dark:bg-slate-900 dark:text-slate-50">
  <h1 class="text-3xl font-bold mb-8">Log In</h1>
  <!-- Contenedor en fila -->
  <button
    on:click={intw}
    class="flex items-center gap-3 px-4 py-2 bg-purple-800 text-slate-50 rounded-sm hover:bg-purple-700 active:bg-purple-950 text-sm sm:text-base md:text-lg">
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 2400 2800" class="w-6 h-6 fill-current">
      <path d="M0 0v2200h600v600l600-600h600l600-600V0H0zm2100 1200l-300 300h-600l-300 300v-300H600V300h1500v900z"/>
      <path d="M1800 600h-300v600h300V600zm-600 0h-300v600h300V600z"/>
    </svg>
    <span class="leading-none font-normal">Log in with Twitch</span>
  </button>
</div>
