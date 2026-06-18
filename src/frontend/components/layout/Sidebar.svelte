<script lang="ts">
  import { onMount } from 'svelte';
  import { connectionStore } from '../../stores/connectionStore';
  import ConnectionTree from '../connection/ConnectionTree.svelte';
  import ConnectionDialog from '../connection/ConnectionDialog.svelte';
  import DatabaseTree from '../explorer/DatabaseTree.svelte';
  import type { Connection } from '../../types/connection';

  let { onSelectCollection }: {
    onSelectCollection: (db: string, coll: string) => void;
  } = $props();

  let showAddDialog = $state(false);
  let searchQuery = $state('');
  let connections = $state<Connection[]>([]);
  let loading = $state(false);
  let activeDatabase = $state('');
  let activeCollection = $state('');

  let filteredConnections = $derived(
    searchQuery
      ? connections.filter(c => c.name.toLowerCase().includes(searchQuery.toLowerCase()))
      : connections
  );

  async function refresh() {
    loading = true;
    await connectionStore.loadConnections();
    connections = [...connectionStore.connections];
    loading = false;
  }

  onMount(async () => {
    await refresh();
    await connectionStore.loadGroups();
  });

  async function handleAddConnection(conn: any) {
    await connectionStore.addConnection(conn);
    showAddDialog = false;
    await refresh();
  }

  function handleSelectConnection(id: string) {
    connectionStore.setActiveConnection(id);
  }

  function handleSelectCollection(db: string, coll: string) {
    activeDatabase = db;
    activeCollection = coll;
    onSelectCollection(db, coll);
  }
</script>

<aside class="flex h-full w-[240px] flex-col border-r border-[var(--border-subtle)] bg-[var(--bg-panel)]">
  <div class="flex items-center justify-between border-b border-[var(--border-subtle)] px-3 py-2">
    <span class="text-[11px] font-medium uppercase tracking-wider text-[var(--text-tertiary)]">Connections</span>
    <div class="flex items-center gap-0.5">
      <button
        aria-label="Add connection"
        class="rounded p-1 text-[var(--text-tertiary)] transition-colors hover:bg-[var(--bg-surface)] hover:text-[var(--text-secondary)]"
        onclick={() => showAddDialog = true}
      >
        <svg class="h-3.5 w-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
          <path stroke-linecap="round" d="M12 4v16m8-8H4" />
        </svg>
      </button>
      <button
        aria-label="Refresh"
        class="rounded p-1 text-[var(--text-tertiary)] transition-colors hover:bg-[var(--bg-surface)] hover:text-[var(--text-secondary)]"
        onclick={refresh}
      >
        <svg class="h-3.5 w-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
          <path stroke-linecap="round" stroke-linejoin="round" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
        </svg>
      </button>
    </div>
  </div>

  <div class="flex-1 overflow-y-auto px-1.5 py-1">
    <div class="mb-2 px-1.5">
      <input
        type="text"
        bind:value={searchQuery}
        placeholder="Search..."
        class="w-full rounded border border-[var(--border-subtle)] bg-[var(--bg-surface)] px-2 py-1 text-[11px] text-[var(--text-secondary)] placeholder-[var(--text-tertiary)] outline-none focus:border-emerald-500"
      />
    </div>

    {#if loading}
      <div class="px-3 py-4 text-center text-[11px] text-[var(--text-tertiary)]">Loading...</div>
    {:else}
      <ConnectionTree
        connections={filteredConnections}
        activeConnectionId={connectionStore.activeConnectionId}
        onSelect={handleSelectConnection}
      />

      <DatabaseTree
        connectionId={connectionStore.activeConnectionId || ''}
        {activeDatabase}
        {activeCollection}
        onSelectCollection={handleSelectCollection}
      />
    {/if}
  </div>

  <div class="border-t border-[var(--border-subtle)] p-2">
    <button
      class="flex w-full items-center justify-center gap-2 rounded-md bg-emerald-500 px-3 py-2 text-[12px] font-medium text-white transition-colors hover:bg-emerald-400"
      onclick={() => showAddDialog = true}
    >
      <svg class="h-3.5 w-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
        <path stroke-linecap="round" d="M12 4v16m8-8H4" />
      </svg>
      New Connection
    </button>
  </div>
</aside>

<ConnectionDialog bind:open={showAddDialog} onSave={handleAddConnection} />
