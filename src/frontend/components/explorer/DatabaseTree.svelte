<script lang="ts">
  import type { DatabaseInfo, CollectionInfo } from '../../types/document';
  import { tauriInvoke } from '../../services/tauriBridge';
  import DatabaseItem from './DatabaseItem.svelte';
  import CollectionList from './CollectionList.svelte';

  let { connectionId, activeDatabase, activeCollection, onSelectCollection }: {
    connectionId: string;
    activeDatabase: string;
    activeCollection: string;
    onSelectCollection: (db: string, coll: string) => void;
  } = $props();

  let databases = $state<DatabaseInfo[]>([]);
  let collections = $state<Map<string, CollectionInfo[]>>(new Map());
  let loading = $state(false);
  let expandedDbs = $state<Set<string>>(new Set());
  let lastConnectionId = $state('');

  async function refresh() {
    if (!connectionId) {
      databases = [];
      collections = new Map();
      expandedDbs = new Set();
      return;
    }
    loading = true;
    const { data, error } = await tauriInvoke<DatabaseInfo[]>('list_databases', { connectionId });
    if (!error && data) {
      databases = data;
    } else {
      databases = [];
    }
    loading = false;
  }

  async function loadCollections(db: string) {
    const { data, error } = await tauriInvoke<CollectionInfo[]>('list_collections', { connectionId, database: db });
    if (!error && data) {
      collections = new Map([[db, data]]);
    }
  }

  async function toggleDatabase(db: string) {
    if (expandedDbs.has(db)) {
      expandedDbs.delete(db);
      expandedDbs = new Set(expandedDbs);
    } else {
      expandedDbs.add(db);
      expandedDbs = new Set(expandedDbs);
      if (!collections.has(db)) {
        await loadCollections(db);
      }
    }
  }

  $effect(() => {
    if (connectionId && connectionId !== lastConnectionId) {
      lastConnectionId = connectionId;
      databases = [];
      collections = new Map();
      expandedDbs = new Set();
      refresh();
    }
  });
</script>

{#if connectionId}
  <div class="mt-2 border-t border-[#2D3A45] pt-2">
    <div class="mb-1 flex items-center justify-between px-2">
      <span class="text-[10px] font-semibold uppercase tracking-wider text-[#7E97A7]">Databases</span>
      <button
        class="rounded p-0.5 text-[#7E97A7] hover:bg-[#1F2933] hover:text-[#C3D4DE] transition-colors"
        onclick={refresh}
        aria-label="Refresh databases"
      >
        <svg class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
          <path stroke-linecap="round" stroke-linejoin="round" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
        </svg>
      </button>
    </div>

    {#if loading}
      <div class="px-3 py-2 text-center text-[11px] text-[#7E97A7]">Loading...</div>
    {:else if databases.length === 0}
      <div class="px-3 py-2 text-center text-[11px] text-[#7E97A7]">No databases</div>
    {:else}
      {#each databases as db (db.name)}
        <DatabaseItem
          database={db}
          {connectionId}
          isActive={activeDatabase === db.name}
          onSelect={() => {}}
          onToggle={() => toggleDatabase(db.name)}
        />
        {#if expandedDbs.has(db.name) && collections.has(db.name)}
          <CollectionList
            database={db.name}
            collections={collections.get(db.name) || []}
            onSelectCollection={onSelectCollection}
          />
        {/if}
      {/each}
    {/if}
  </div>
{/if}
