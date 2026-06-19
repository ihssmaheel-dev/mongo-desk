<script lang="ts">
  import { onMount } from 'svelte';
  import { connectionStore } from '../../stores/connectionStore';
  import ConnectionTree from '../connection/ConnectionTree.svelte';
  import ConnectionDialog from '../connection/ConnectionDialog.svelte';
  import ConfirmDialog from '../common/ConfirmDialog.svelte';
  import DatabaseTree from '../explorer/DatabaseTree.svelte';
  import type { Connection } from '../../types/connection';

  let { onSelectCollection, onSelectConnection }: {
    onSelectCollection: (db: string, coll: string) => void;
    onSelectConnection: (id: string) => void;
  } = $props();

  let showAddDialog = $state(false);
  let showDeleteConfirm = $state(false);
  let deleteTargetId = $state('');
  let deleteTargetName = $state('');
  let searchQuery = $state('');
  let connections = $state<Connection[]>([]);
  let loading = $state(false);
  let activeDatabase = $state('');
  let activeCollection = $state('');
  let activeConnectionId = $state('');

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
  });

  async function handleAddConnection(conn: any) {
    await connectionStore.addConnection(conn);
    showAddDialog = false;
    await refresh();
  }

  function handleSelectConnection(id: string) {
    activeConnectionId = id;
    connectionStore.setActiveConnection(id);
    activeDatabase = '';
    activeCollection = '';
    onSelectConnection(id);
  }

  function handleSelectCollection(db: string, coll: string) {
    activeDatabase = db;
    activeCollection = coll;
    onSelectCollection(db, coll);
  }

  function confirmDelete(id: string) {
    const conn = connections.find(c => c.id === id);
    if (conn) {
      deleteTargetId = id;
      deleteTargetName = conn.name;
      showDeleteConfirm = true;
    }
  }

  async function handleDelete() {
    await connectionStore.deleteConnection(deleteTargetId);
    if (activeConnectionId === deleteTargetId) {
      activeConnectionId = '';
      activeDatabase = '';
      activeCollection = '';
      onSelectConnection('');
    }
    showDeleteConfirm = false;
    await refresh();
  }
</script>

<aside class="flex h-full w-[220px] flex-col border-r border-[#2D3A45] bg-[#161D24]">
  <div class="flex items-center justify-between border-b border-[#2D3A45] px-3 py-2">
    <span class="text-[10px] font-semibold uppercase tracking-wider text-[#465A6B]">Connections</span>
    <div class="flex items-center gap-0.5">
      <button
        aria-label="Add connection"
        class="rounded p-1 text-[#465A6B] transition-colors hover:bg-[#1F2933] hover:text-[#C3D4DE]"
        onclick={() => showAddDialog = true}
      >
        <svg class="h-3.5 w-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
          <path stroke-linecap="round" d="M12 4v16m8-8H4" />
        </svg>
      </button>
      <button
        aria-label="Refresh"
        class="rounded p-1 text-[#465A6B] transition-colors hover:bg-[#1F2933] hover:text-[#C3D4DE]"
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
        class="w-full rounded border border-[#2D3A45] bg-[#0E1318] px-2 py-1 text-[11px] text-[#C3D4DE] placeholder-[#465A6B] outline-none focus:border-[#00ED64] transition-colors"
      />
    </div>

    {#if loading}
      <div class="px-3 py-4 text-center text-[11px] text-[#465A6B]">Loading...</div>
    {:else}
      <ConnectionTree
        connections={filteredConnections}
        {activeConnectionId}
        onSelect={handleSelectConnection}
        onDelete={confirmDelete}
      />

      <DatabaseTree
        connectionId={activeConnectionId}
        {activeDatabase}
        {activeCollection}
        onSelectCollection={handleSelectCollection}
      />
    {/if}
  </div>

  <div class="border-t border-[#2D3A45] p-2">
    <button
      class="flex w-full items-center justify-center gap-2 rounded bg-[#00684A] px-3 py-2 text-[11px] font-medium text-white transition-colors hover:bg-[#00C75A]"
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

<ConfirmDialog
  bind:open={showDeleteConfirm}
  title="Delete Connection"
  message="Are you sure you want to delete '{deleteTargetName}'? This action cannot be undone."
  confirmText="Delete"
  variant="danger"
  onConfirm={handleDelete}
/>
