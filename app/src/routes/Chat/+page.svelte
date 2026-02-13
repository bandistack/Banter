<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/core";
  import Navbar from "../Navbar/+page.svelte";
  import MessageInput from "../MessageInput/+page.svelte";
  import Logout from "../Logout/+page.svelte";
  import Badges from "../Badges/+page.svelte";

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

  onMount(async () => {
    try {
      currentUser = await invoke("get_current_user");
    } catch (err) {
      console.error("Failed to get current user:", err);
    }

    unsubscribe = await listen<ChatMessage>("twitch:message", (event) => {
      messages = [...messages.slice(-200), event.payload];
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

  async function sendMessage() {
    if (!newMessage.trim()) return;
    const text = newMessage;
    messages = [...messages, { username: currentUser, display_name: currentUser, text, badges: [], color: null }];
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

    <div class="flex-1 p-4 space-y-2 overflow-y-auto scrollbar-hide" use:autoScroll>
      {#each messages as msg}
        <div class="flex flex-col gap-1 bg-slate-100 dark:bg-slate-950 rounded-sm p-2">
          <div class="flex items-start gap-2 p-2">
            <span class="font-bold" style={msg.color ? `color: ${msg.color}` : ""}>{msg.display_name}</span>
            <span class="break-all max-w-full overflow-hidden">{msg.text}</span>
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
      <span class="px-4 py-1 text-slate-400 dark:stroke-slate-600">Chat settings</span>
      <Badges />
      <Logout />
    </div>
  {/if}
</div>