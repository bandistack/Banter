<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/core";
  import Navbar from "../Navbar/+page.svelte";
  import MessageInput from "../MessageInput/+page.svelte";
  import Logout from "../Logout/+page.svelte";
  import Badges from "../Badges/+page.svelte";
  import { badgesEnabled, badgeCache } from "$lib/store";

  interface ChatMessage {
    username: string;
    display_name: string;
    text: string;
    badges: string[];
    color: string | null;
  }

  let messages: ChatMessage[] = [];
  let newMessage = "";
  let menuOpen = false;
  let currentUser = "";
  let unsubscribe: (() => void) | null = null;

  // Resuelve badges que aún no están en el caché local
  async function resolveMissingBadges(badgeIds: string[]) {
    if (!badgeIds.length) return;
    const cache = $badgeCache;
    const missing = badgeIds.filter((id) => {
      const key = id.includes("/") ? id : `${id}/1`;
      return !cache[key] && !cache[id];
    });
    if (!missing.length) return;
    try {
      const resolved = await invoke<Record<string, { url_1x: string; url_2x: string }>>(
        "resolve_badges",
        { badgeIds: missing }
      );
      badgeCache.update((c) => ({ ...c, ...resolved }));
    } catch (err) {
      console.error("Error resolviendo badges:", err);
    }
  }

  onMount(async () => {
    try {
      currentUser = await invoke("get_current_user");
    } catch (err) {
      console.error("Failed to get current user:", err);
    }

    unsubscribe = await listen<ChatMessage>("twitch:message", (event) => {
      const msg = event.payload;
      // resolver badges de este mensaje en background
      resolveMissingBadges(msg.badges);
      messages = [...messages.slice(-200), msg];
    });

    invoke("twitch_connect", { channel: currentUser }).catch((err) => {
      console.error("Failed to connect to Twitch:", err);
    });
  });

  onDestroy(() => {
    unsubscribe?.();
    invoke("twitch_disconnect").catch(() => {});
  });

  function autoScroll(node: HTMLDivElement) {
    const observer = new MutationObserver(() => { node.scrollTop = node.scrollHeight; });
    observer.observe(node, { childList: true });
    return { destroy() { observer.disconnect(); } };
  }

  // Obtiene la URL del badge del caché local (reactivo)
  function getBadgeUrl(badge: string): string | null {
    const key = badge.includes("/") ? badge : `${badge}/1`;
    return $badgeCache[key]?.url_1x ?? $badgeCache[badge]?.url_1x ?? null;
  }

  async function sendMessage() {
    if (!newMessage.trim()) return;
    const text = newMessage;
    messages = [...messages.slice(-200), {
      username: currentUser,
      display_name: currentUser,
      text,
      badges: [],
      color: null,
    }];
    newMessage = "";
    try {
      await invoke("twitch_send", { text });
    } catch (err) {
      console.error("Error enviando mensaje:", err);
    }
  }
</script>

<div class="flex h-screen w-screen bg-slate-50 text-slate-900 dark:bg-slate-900 dark:text-slate-50">
  <div class="flex flex-col flex-1">
    <Navbar bind:open={menuOpen} />

    <div class="flex-1 px-3 overflow-y-auto 
      [&::-webkit-scrollbar]:w-3
    [&::-webkit-scrollbar-track]:bg-slate-50
    [&::-webkit-scrollbar-thumb]:bg-slate-200
      [&::-webkit-scrollbar-thumb]:rounded-sm
    dark:[&::-webkit-scrollbar-track]:bg-slate-900
    dark:[&::-webkit-scrollbar-thumb]:bg-slate-800" use:autoScroll>
      {#each messages as msg}
        <div class="flex flex-col gap-1 bg-slate-100 dark:bg-slate-950 rounded-sm py-1 px-2 my-2">
          <div class="flex items-center gap-1 flex-wrap px-2 py-1">

            <!-- Badges -->
            {#if $badgesEnabled && msg.badges.length > 0}
              {#each msg.badges as badge}
                {#if getBadgeUrl(badge)}
                  <img
                    src={getBadgeUrl(badge)}
                    alt={badge}
                    title={badge}
                    class="w-4 h-4 inline-block shrink-0"
                  />
                {/if}
              {/each}
            {/if}

            <!-- Nombre -->
            <span
              class="font-bold shrink-0 text-slate-700  dark:text-slate-300"
              style={msg.color ? `color: ${msg.color}` : ""}>
              {msg.display_name}:
            </span>

            <!-- Texto -->
            <span class="break-all text-slate-700  dark:text-slate-300">{msg.text}</span>

          </div>
        </div>
      {/each}
    </div>

    <MessageInput
      bind:value={newMessage}
      placeholder="Escribe un mensaje..."
      onSend={sendMessage}
    />
  </div>

  {#if menuOpen}
    <div class="flex flex-col justify-start w-50 h-full bg-slate-100 dark:bg-slate-950">
      <span class="px-4 py-1 text-slate-400 dark:stroke-slate-600">Settings</span>
      <div class="py-2 m-4 bg-slate-200 dark:bg-slate-800 rounded-sm">
        <Badges />
      </div>
      <Logout />
    </div>
  {/if}
</div>
