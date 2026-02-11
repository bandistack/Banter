<script lang="ts">
  import { onMount, onDestroy, tick } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/core";
  import { UserTokenPersistence } from "$lib/tokenpersistence";
  import { ensureTokenLife } from "$lib/tokenlife";
  import Navbar from "../Navbar/+page.svelte";
  import MessageInput from "../MessageInput/+page.svelte";
  import Logout from "../Logout/+page.svelte";
  import Badges from "../Badges/+page.svelte";

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
  async function sendMessage() {
    if (!newMessage.trim()) return;
    try {
      // Mostrar el mensaje inmediatamente en la UI
      messages = [
        ...messages,
        {
          nick: currentUser,
          message: newMessage,
          badges: null,
          color: null,
          raw: ""
        }
      ];
      // Enviar al backend
      await invoke("send_twitch_message", {
        channel: currentUser,
        msg: newMessage
      });
      newMessage = ""; // limpiar input
    } catch (err) {
      console.error("Error enviando mensaje:", err);
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
          <div class="flex items-start gap-2 bg-slate-100 dark:bg-slate-950 rounded-sm p-2">
            <span class="font-bold" style={msg.color ? `color: ${msg.color}` : ""}>{msg.nick}</span>
            <span class="break-all max-w-full overflow-hidden">{msg.message}</span>
          </div>
        </div>
      {/each}
    </div>
    <!-- Input -->
     <MessageInput
        bind:value={newMessage}
        placeholder="Escribe un mensaje..."
        onSend={sendMessage}
      />
  </div>
  <!-- Menú lateral como hijo directo del flex -->
  {#if menuOpen}
    <div class="flex flex-col justify-start w-50 h-full bg-slate-100 dark:bg-slate-950 ">
      <span class="px-4 py-1 text-slate-400 dark:stroke-slate-600">Chat settings</span>
      <Badges />
      <Logout />
    </div>
  {/if}
</div>

