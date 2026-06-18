<script lang="ts">
  import type { CollectionInfo } from '../../types/document';

  let { collection, database, onSelect }: {
    collection: CollectionInfo;
    database: string;
    onSelect: (db: string, coll: string) => void;
  } = $props();
</script>

<button
  class="group flex w-full items-center gap-2 rounded px-2 py-[5px] text-left text-[12px] text-[var(--text-secondary)] transition-all duration-100 hover:bg-[var(--bg-surface)] hover:text-[var(--text-primary)]"
  onclick={() => onSelect(database, collection.name)}
>
  <svg class="h-3.5 w-3.5 text-[var(--text-tertiary)] group-hover:text-[var(--text-secondary)]" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
    <path stroke-linecap="round" stroke-linejoin="round" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
  </svg>
  <span class="truncate">{collection.name}</span>
</button>
