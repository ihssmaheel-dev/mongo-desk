<script lang="ts">
  import { errorStore } from '../../stores/errorStore.svelte';
  import Toast from './Toast.svelte';
</script>

<div class="fixed bottom-4 right-4 z-50 flex flex-col gap-2 max-w-sm">
  {#each errorStore.transient as entry (entry.id)}
    <Toast {entry} onDismiss={(id) => errorStore.dismiss(id)} />
  {/each}
  {#each errorStore.persistent as entry (entry.id)}
    <Toast {entry} onDismiss={(id) => errorStore.dismiss(id)} />
  {/each}
</div>
