<script lang="ts">
  export let open: boolean;
</script>


<button 
  on:click={() => open = !open} 
  aria-expanded={open}
  aria-label="menu">
  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" class="w-6 h-6 stroke-slate-400 hover:stroke-slate-500 dark:stroke-slate-600 active:stroke-slate-200 active:dark:stroke-slate-800 stroke-2 fill-none"> <rect x="3" y="3" width="18" height="18" rx="2" /> <path d="M15 3v18" /> </svg>
</button>