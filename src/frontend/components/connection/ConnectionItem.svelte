<script lang="ts">
  import type { Connection } from '../../types/connection';
  
  let { connection, isActive = false, onClick }: { connection: Connection; isActive?: boolean; onClick: () => void } = $props();
  
  const statusColor = $derived(connection.read_only ? 'bg-semantic-info' : 'bg-semantic-success');
</script>

<button
  class="flex w-full items-center gap-2 rounded-sm px-3 py-2 text-left text-sm transition-colors hover:bg-slate-100 dark:hover:bg-slate-800 {isActive ? 'bg-brand-mist dark:bg-brand-forest/30' : ''}"
  onclick={onClick}
>
  <div class="h-2 w-2 rounded-full {statusColor}"></div>
  <span class="flex-1 truncate text-slate-900 dark:text-slate-100">{connection.name}</span>
  {#if connection.read_only}
    <span class="rounded bg-semantic-info/20 px-1.5 py-0.5 text-xs text-semantic-info">RO</span>
  {/if}
</button>
