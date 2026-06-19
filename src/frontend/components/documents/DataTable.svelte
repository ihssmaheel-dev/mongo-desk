<script lang="ts">
  let {
    data = [],
    columns = [],
    onRowClick,
    onSort,
    onFilter,
    sortField = '',
    sortDir = 'asc' as 'asc' | 'desc',
    filters = {},
  }: {
    data: any[];
    columns: string[];
    onRowClick?: (row: any, index: number) => void;
    onSort?: (field: string, dir: 'asc' | 'desc' | null) => void;
    onFilter?: (col: string) => void;
    sortField?: string;
    sortDir?: 'asc' | 'desc';
    filters?: Record<string, any>;
  } = $props();

  function handleSort(col: string) {
    if (sortField === col) {
      if (sortDir === 'asc') onSort?.(col, 'desc');
      else onSort?.('', null);
    } else {
      onSort?.(col, 'asc');
    }
  }

  function getValueClass(val: any): string {
    if (val === null || val === undefined) return 'text-[#7E97A7] italic';
    if (typeof val === 'object' && val.$oid) return 'text-[#FFC010] font-mono';
    if (typeof val === 'object' && val.$date) return 'text-[#B79CFF]';
    if (Array.isArray(val)) return 'text-[#7E97A7]';
    if (typeof val === 'object') return 'text-[#7E97A7]';
    if (typeof val === 'number') return 'text-[#5DD0FF]';
    if (typeof val === 'boolean') return 'text-[#FF8966]';
    return 'text-[#00ED64]';
  }

  function formatValue(val: any): string {
    if (val === null || val === undefined) return 'null';
    if (typeof val === 'object' && val.$oid) return val.$oid;
    if (typeof val === 'object' && val.$date) {
      try { return new Date(val.$date.$numberLong || val.$date).toLocaleString(); } catch { return String(val.$date); }
    }
    if (typeof val === 'object' && Array.isArray(val)) return `[${val.length} items]`;
    if (typeof val === 'object') return '{...}';
    return String(val);
  }
</script>

<div class="overflow-auto h-full custom-scrollbar">
  <table class="w-full border-collapse min-w-max">
    <thead class="sticky top-0 z-10 bg-[#0E1318]">
      <tr>
        {#each columns as col}
          <th class="border-b border-r border-[#2D3A45] px-3 py-2 text-left text-[10px] font-semibold text-[#7E97A7] whitespace-nowrap last:border-r-0 group">
            <div class="flex items-center justify-between gap-1">
              <button class="hover:text-[#C3D4DE] transition-colors {sortField === col ? 'text-[#00ED64]' : ''}" onclick={() => handleSort(col)}>
                <span>{col === '_id' ? '_id' : col}</span>
                {#if sortField === col}
                  <span class="text-[#00ED64] text-[9px] ml-0.5">{sortDir === 'asc' ? '↑' : '↓'}</span>
                {/if}
              </button>
              <button class="opacity-0 group-hover:opacity-100 {filters[col] ? 'opacity-100 !text-[#00ED64]' : 'text-[#465A6B]'} hover:text-[#C3D4DE] transition-all" onclick={() => onFilter?.(col)}>
                <svg class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" d="M3 4a1 1 0 011-1h16a1 1 0 011 1v2.586a1 1 0 01-.293.707l-6.414 6.414a1 1 0 00-.293.707V17l-4 4v-6.586a1 1 0 00-.293-.707L3.293 7.293A1 1 0 013 6.586V4z"/></svg>
              </button>
            </div>
          </th>
        {/each}
      </tr>
    </thead>
    <tbody>
      {#each data as doc, idx}
        <tr class="cursor-pointer border-b border-[#1F2933] hover:bg-[#161D24] transition-colors" onclick={() => onRowClick?.(doc, idx)}>
          {#each columns as col}
            {@const val = doc[col]}
            <td class="border-b border-r border-[#1F2933] px-3 py-1.5 text-[11px] {getValueClass(val)} last:border-r-0">
              <span class="max-w-[200px] truncate block">{formatValue(val)}</span>
            </td>
          {/each}
        </tr>
      {/each}
    </tbody>
  </table>
</div>
