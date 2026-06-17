<script lang="ts">
  import type { Connection } from '../../types/connection';
  import ConnectionTree from '../connection/ConnectionTree.svelte';
  import ConnectionDialog from '../connection/ConnectionDialog.svelte';
  
  let { connections = [], activeConnectionId = null, onSelectConnection, onAddConnection }: {
    connections: Connection[];
    activeConnectionId?: string | null;
    onSelectConnection: (id: string) => void;
    onAddConnection: (conn: any) => void;
  } = $props();
  
  let showAddDialog = $state(false);
  let searchQuery = $state('');
  
  let filteredConnections = $derived(
    searchQuery
      ? connections.filter(c => c.name.toLowerCase().includes(searchQuery.toLowerCase()))
      : connections
  );
</script>

<div class="flex h-full w-64 flex-col border-r border-slate-200 bg-slate-50 dark:border-slate-700 dark:bg-slate-900">
  <div class="border-b border-slate-200 p-2 dark:border-slate-700">
    <input
      type="text"
      bind:value={searchQuery}
      placeholder="Search connections..."
      class="w-full rounded-sm border border-slate-200 bg-white px-3 py-1.5 text-xs text-slate-900 placeholder-slate-400 focus:border-brand-evergreen focus:outline-none focus:ring-1 focus:ring-brand-evergreen dark:border-slate-700 dark:bg-slate-800 dark:text-slate-100"
    />
  </div>
  
  <div class="flex-1 overflow-y-auto">
    <ConnectionTree
      connections={filteredConnections}
      {activeConnectionId}
      onSelect={onSelectConnection}
    />
  </div>
  
  <div class="border-t border-slate-200 p-2 dark:border-slate-700">
    <button
      class="flex w-full items-center justify-center gap-2 rounded-sm bg-brand-evergreen px-3 py-2 text-xs font-medium text-white hover:bg-brand-spring-dim"
      onclick={() => showAddDialog = true}
    >
      <svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
      </svg>
      New Connection
    </button>
  </div>
</div>

<ConnectionDialog bind:open={showAddDialog} onSave={onAddConnection} />
