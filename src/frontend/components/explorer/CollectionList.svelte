<script lang="ts">
  import type { CollectionInfo } from '../../types/document';
  import CollectionItem from './CollectionItem.svelte';

  let { database, collections, onSelectCollection }: {
    database: string;
    collections: CollectionInfo[];
    onSelectCollection: (db: string, coll: string) => void;
  } = $props();
</script>

<div class="ml-4 border-l border-[var(--border-subtle)] pl-1.5">
  {#each collections as collection (collection.name)}
    <CollectionItem {collection} {database} onSelect={onSelectCollection} />
  {/each}

  {#if collections.length === 0}
    <div class="px-2 py-1 text-[11px] text-[var(--text-tertiary)]">No collections</div>
  {/if}
</div>
