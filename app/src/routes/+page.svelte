<script lang="ts">
  import { goto } from '$app/navigation';
  import { statusMessage } from "$lib/status";
  import { UserTokenPersistence } from "$lib/persistence";

  const storedToken = UserTokenPersistence.load();
    if (storedToken?.access_token) {
      statusMessage.set({ type: "success", text: storedToken.access_token });
      goto("/Chat");
    } 
    else {
      statusMessage.set({ type: "success", text: "Hey" });
      goto('/Login');
    }
</script>