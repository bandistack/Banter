<script lang="ts">
  import { statusMessage } from "$lib/status";
</script>

{#if $statusMessage}
  <div
    class={`fixed bottom-4 left-1/2 transform -translate-x-1/2 px-4 py-2 rounded shadow-lg text-sm z-50
      ${$statusMessage.type === "success" ? "bg-green-900 text-slate-50" : "bg-red-900 text-slate-50"}`}>
    {$statusMessage.text}
  </div>
{/if}
