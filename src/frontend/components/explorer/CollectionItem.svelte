<script lang="ts">
  import type { CollectionInfo } from '../../types/document';

  let { collection, database, onSelect }: {
    collection: CollectionInfo;
    database: string;
    onSelect: (db: string, coll: string) => void;
  } = $props();
</script>

<button
  class="flex w-full items-center gap-2 rounded px-2 py-1 text-left text-[12px] text-[#C3D4DE] transition-colors hover:bg-[#1F2933]"
  onclick={() => onSelect(database, collection.name)}
>
  <svg class="h-3.5 w-3.5 flex-shrink-0 text-[#7E97A7]" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
    <path stroke-linecap="round" stroke-linejoin="round" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
  </svg>
  <span class="truncate">{collection.name}</span>
</button>
