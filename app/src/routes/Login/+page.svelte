<script lang="ts">
import { getAuthUrl } from '$lib/url';

async function loginWithTwitch(): Promise<void> {
    window.location.href = getAuthUrl();
  }
</script>
<div class="flex flex-col h-screen w-screen justify-center items-center p-10 bg-gray-900 text-slate-50">
  <h1 class="text-3xl font-bold mb-8">Log in</h1>
  <button
    on:click={loginWithTwitch}
    class="flex items-center gap-3 px-4 py-2 bg-purple-800 text-slate-100 rounded-sm hover:bg-purple-700 transition text-sm sm:text-base md:text-lg">
    <span class="leading-none">Log in with Twitch</span>
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 2400 2800"
        class="w-6 h-6 fill-current">
      <path d="M0 0v2200h600v600l600-600h600l600-600V0H0zm2100 1200l-300 300h-600l-300 300v-300H600V300h1500v900z"/>
      <path d="M1800 600h-300v600h300V600zm-600 0h-300v600h300V600z"/>
    </svg>
  </button>
</div>