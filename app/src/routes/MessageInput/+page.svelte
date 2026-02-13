<script lang="ts">
  export let value: string = '';
  export let placeholder: string = 'Type something..';
  export let onSend: () => void; // función pasada desde el padre
</script>

<div class="flex px-3 py-2 gap-2">
  <input
    type="text"
    bind:value={value}
    placeholder={placeholder}
    class="flex-1 bg-slate-100 dark:bg-slate-950 rounded-sm px-3 focus:outline-none focus:bg-slate-200 focus:dark:bg-slate-800"
    on:keydown={(e) => e.key === "Enter" && onSend()}
  />
  <button
    type="button"
    on:click={onSend}
    class="bg-slate-200 dark:bg-slate-800 px-3 py-2 rounded-sm hover:bg-slate-300 hover:dark:bg-slate-700 active:bg-slate-100 active:dark:bg-slate-900 border-none"
  >
    <span class="text-slate-700 dark:text-slate-300">Send</span>
  </button>
</div>
