<script>
  let { children } = $props();
  import "../app.css";
  import Status from "$lib/Components/status.svelte";
</script>

{@render children()}
<Status />