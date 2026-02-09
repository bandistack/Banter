<script>
  import { onMount } from "svelte";
  let { children } = $props();
  import "../app.css";

  onMount(() => {
    const prefersDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
    const theme = localStorage.theme;

    document.documentElement.classList.toggle(
      "dark",
      theme === "dark" || (!theme && prefersDark)
    );
  });
</script>

{@render children()}
