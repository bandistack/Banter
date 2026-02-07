<script lang="ts">
  import { goto } from '$app/navigation';
  import { UserTokenPersistence } from "$lib/tokenpersistence";

  const storedToken = UserTokenPersistence.load();
    if (storedToken?.access_token) {
      goto("/Chat");
    } 
    else {
      goto('/Login');
    }
</script>