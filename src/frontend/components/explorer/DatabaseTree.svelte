<script lang="ts">
  import type { DatabaseInfo } from '../../types/document';
  import type { CollectionInfo } from '../../types/document';
  import DatabaseItem from './DatabaseItem.svelte';
  import CollectionList from './CollectionList.svelte';
  import { explorerStore } from '../../stores/explorerStore';

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

  async function refresh() {
    loading = true;
    await explorerStore.loadDatabases(connectionId);
    databases = [...explorerStore.databases];
    loading = false;
  }

  async function toggleDatabase(db: string) {
    if (expandedDbs.has(db)) {
      expandedDbs.delete(db);
      expandedDbs = new Set(expandedDbs);
    } else {
      expandedDbs.add(db);
      expandedDbs = new Set(expandedDbs);
      await explorerStore.loadCollections(connectionId, db);
      collections = new Map(explorerStore.collections);
    }
  }

  $effect(() => {
    if (connectionId) {
      refresh();
    } else {
      databases = [];
      collections = new Map();
      expandedDbs = new Set();
    }
  });
</script>

{#if connectionId}
  <div class="mt-2 border-t border-[var(--border-subtle)] pt-2">
    <div class="mb-1 flex items-center justify-between px-2">
      <span class="text-[10px] font-semibold uppercase tracking-wider text-[var(--text-tertiary)]">Databases</span>
      <button
        class="rounded p-0.5 text-[var(--text-tertiary)] hover:bg-[var(--bg-surface)] hover:text-[var(--text-secondary)]"
        onclick={refresh}
        aria-label="Refresh databases"
      >
        <svg class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
          <path stroke-linecap="round" stroke-linejoin="round" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
        </svg>
      </button>
    </div>

    {#if loading}
      <div class="px-3 py-2 text-center text-[11px] text-[var(--text-tertiary)]">Loading...</div>
    {:else if databases.length === 0}
      <div class="px-3 py-2 text-center text-[11px] text-[var(--text-tertiary)]">No databases</div>
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
