<script lang="ts">
  import { onMount, onDestroy, tick } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/core";
  import { UserTokenPersistence } from "$lib/tokenpersistence";
  import { ensureTokenLife } from "$lib/tokenlife";
  import Navbar from "../Navbar/+page.svelte";
  import MessageInput from "../MessageInput/+page.svelte";

  interface ChatMessage {
    nick: string;
    message: string;
    badges: string | null;
    color: string | null;
    raw: string;
  }

  let messages: ChatMessage[] = [];
  let newMessage = "";
  let menuOpen = false;
  let currentUser = "";
  let unsubscribe: (() => void) | null = null;

  onMount(async () => {
    ensureTokenLife(UserTokenPersistence.load());
    try {
      currentUser = await invoke("get_current_user");
    } catch (err) {
      console.error("Failed to get current user:", err);
    }

    unsubscribe = await listen("twitch_message", (event) => {
      try {
        const chatMsg = JSON.parse(event.payload as string) as ChatMessage;
        messages = [...messages, chatMsg];
      } catch (e) {
        console.error("Failed to parse message:", e);
      }
    });

    try {
      invoke("start_twitch_chat").catch((err) => {
        console.error("Failed to start twitch chat:", err);
      });
    } catch (err) {
      console.error("Failed to invoke start_twitch_chat:", err);
    }
  });

  onDestroy(() => {
    unsubscribe?.();
  });
  function autoScroll(node: HTMLDivElement) {
    const observer = new MutationObserver(() => {
      node.scrollTop = node.scrollHeight;
    });
    observer.observe(node, { childList: true });
    return {
      destroy() {
        observer.disconnect();
      }
    };
  }
  function sendMessage() {
    if (newMessage.trim() !== "") {
      const userMsg: ChatMessage = {
        nick: currentUser || "Usuario",
        message: newMessage,
        badges: null,
        color: null,
        raw: ""
      };
      messages = [...messages, userMsg];
      newMessage = "";
    }
  }
</script>
<div class="flex h-screen w-screen bg-slate-50 text-slate-900 dark:bg-slate-900 dark:text-slate-50">
  <!-- Contenedor principal en flex -->
  <div class="flex flex-col flex-1">
    <!-- Navbar controla el toggle -->
    <Navbar bind:open={menuOpen} />

    <!-- Ventana de mensajes -->
    <div class="flex-1 p-4 space-y-2 overflow-y-auto scrollbar-hide" use:autoScroll>
      {#each messages as msg}
        <div class="flex flex-col gap-1 bg-slate-100 dark:bg-slate-950 rounded-sm p-2">
          <span class="text-slate-500 text-xs dark:text-slate-400 px-2">{msg.badges?.split(",").map(b => b.split("/")[0]).join(" ")}</span>
          <div class="flex items-start gap-2 bg-slate-100 dark:bg-slate-950 rounded-sm p-2">
            <span class="font-bold" style={msg.color ? `color: ${msg.color}` : ""}>{msg.nick}</span>
            <span class="break-all max-w-full overflow-hidden">{msg.message}</span>
          </div>
        </div>
      {/each}
    </div>
    <!-- Input -->
    <MessageInput bind:value={newMessage} on:send={sendMessage} placeholder="Escribe un mensaje..." nick={currentUser} />
  </div>
  <!-- Menú lateral como hijo directo del flex -->
  {#if menuOpen}
    <div class="flex flex-col w-64 h-full bg-slate-100 dark:bg-slate-950">
  
    </div>
  {/if}
</div>

