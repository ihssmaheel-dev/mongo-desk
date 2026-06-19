<script lang="ts">
  import type { Connection } from '../../types/connection';
  import ConnectionItem from './ConnectionItem.svelte';

  let { connections = [], activeConnectionId = null, onSelect, onDelete }: {
    connections: Connection[];
    activeConnectionId?: string | null;
    onSelect: (id: string) => void;
    onDelete: (id: string) => void;
  } = $props();
</script>

<div class="flex flex-col">
  {#each connections as connection (connection.id)}
    <ConnectionItem
      {connection}
      isActive={connection.id === activeConnectionId}
      onClick={() => onSelect(connection.id)}
      {onDelete}
    />
  {/each}

  {#if connections.length === 0}
    <p class="px-3 py-2 text-[11px] text-[#465A6B]">No connections yet</p>
  {/if}
</div>
