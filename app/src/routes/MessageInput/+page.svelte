<script lang="ts">
  export let value: string = '';
  export let placeholder: string = 'Escribe un mensaje...';
  export let nick: string = '';
</script>

<div class="flex px-3 py-2 gap-2">
  {#if nick}
    <div class="flex items-center px-3 py-2 bg-slate-100 dark:bg-slate-950 rounded-sm">{nick}</div>
  {/if}
  <input
    type="text"
    bind:value={value}
    placeholder={placeholder}
    class="flex-1 bg-slate-100 dark:bg-slate-950 rounded-sm px-3 focus:outline-none focus:bg-slate-200 focus:dark:bg-slate-800"
  />
  <button
    class="bg-blue-300 dark:bg-indigo-500 px-3 py-2 rounded-sm hover:bg-blue-400 hover:dark:bg-indigo-400 border-none"
  >
    Send
  </button>
</div>
