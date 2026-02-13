<script lang="ts">
import MenuToggle from '../MenuToggle/+page.svelte';

export let open: boolean;
</script>
<nav class="flex items-center justify-end py-1 px-3 bg-slate-100 dark:bg-slate-950">  
    <div class="flex items-center">
        <MenuToggle bind:open={open} />
    </div>
</nav>
