<script lang="ts">
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/core";
  import { goto } from "$app/navigation";
  import { UserTokenPersistence } from "$lib/tokenpersistence";
  import { ensureTokenLife } from "$lib/tokenlife";

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

  onMount(async () => {
    ensureTokenLife(UserTokenPersistence.load());
    // Obtener nick del usuario actual
    try {
      currentUser = await invoke("get_current_user");
    } catch (err) {
      console.error("Failed to get current user:", err);
    }
    listen("twitch_message", (event) => {
      try {
        const chatMsg = JSON.parse(event.payload as string) as ChatMessage;
        messages = [...messages, chatMsg];
      } catch (e) {
        console.error("Failed to parse message:", e);
      }
    });
    try {
      await invoke("start_twitch_chat");
    } catch (err) {
      console.error("Failed to start twitch chat:", err);
    }
  });

  function sendMessage() {
    if (newMessage.trim() !== "") {
      // TODO: invocar comando Tauri para enviar al IRC
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

  function logout() {
    UserTokenPersistence.clear();
    goto("/Login");
  }
</script>

<div class="flex h-screen bg-gray-100">
  <!-- Menú lateral -->
  <div
    class={`fixed top-0 left-0 h-full w-64 bg-white shadow transform transition-transform duration-300 ${
      menuOpen ? "translate-x-0" : "-translate-x-full"
    }`}
  >
    <div class="p-4 border-b">
      <h2 class="text-lg font-bold">Menú</h2>
    </div>
    <div class="p-4">
      <button
        class="w-full bg-red-500 text-white py-2 rounded hover:bg-red-600"
        on:click={logout}
      >
        Logout
      </button>
    </div>
  </div>

  <!-- Contenido principal -->
  <div class="flex flex-col flex-1">
    <!-- Header con botón menú -->
    <div class="flex items-center justify-between bg-purple-600 text-white px-4 py-3">
      <button on:click={() => (menuOpen = !menuOpen)} class="focus:outline-none">
        ☰
      </button>
      <h1 class="text-lg font-semibold">Chat</h1>
    </div>

    <!-- Ventana de mensajes -->
    <div class="flex-1 overflow-y-auto p-4 space-y-2">
      {#each messages as msg}
        <div class="bg-white shadow rounded px-3 py-2 flex items-start gap-2">
          <span class="text-xs text-gray-500 mt-0.5">{msg.badges ? "⭐" : ""}</span>
          <div class="flex-1">
            <span class="font-bold" style={msg.color ? `color: ${msg.color}` : ""}>{msg.nick}:</span>
            <span>{msg.message}</span>
          </div>
        </div>
      {/each}
    </div>

    <!-- Input + botón Send -->
    <div class="flex items-center border-t bg-white px-4 py-3">
      <input
        type="text"
        bind:value={newMessage}
        placeholder="Escribe un mensaje..."
        class="flex-1 border rounded px-3 py-2 mr-2 focus:outline-none focus:ring focus:border-purple-500"
      />
      <button
        on:click={sendMessage}
        class="bg-purple-600 text-white px-4 py-2 rounded hover:bg-purple-700"
      >
        Send
      </button>
    </div>
  </div>
</div>
