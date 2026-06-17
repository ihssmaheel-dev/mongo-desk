<script lang="ts">
  import { errorStore } from '../../stores/errorStore.svelte';
  import Toast from './Toast.svelte';
</script>

<div class="fixed bottom-4 right-4 z-50 flex flex-col gap-2">
  {#each errorStore.transient as error (error.id)}
    <Toast {error} />
  {/each}
  {#each errorStore.persistent as error (error.id)}
    <Toast {error} />
  {/each}
</div>
