<script lang="ts">
  let {
    data = [],
    columns = [],
    onRowClick,
    onSort,
    sortField = '',
    sortDir = 'asc' as 'asc' | 'desc',
  }: {
    data: any[];
    columns: string[];
    onRowClick?: (row: any, index: number) => void;
    onSort?: (field: string, dir: 'asc' | 'desc' | null) => void;
    sortField?: string;
    sortDir?: 'asc' | 'desc';
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

<div class="overflow-auto">
  <table class="w-full border-collapse">
    <thead class="sticky top-0 z-10 bg-[#0E1318]">
      <tr>
        {#each columns as col}
          <th class="border-b border-r border-[#2D3A45] px-3 py-2 text-left text-[10px] font-semibold text-[#7E97A7] whitespace-nowrap last:border-r-0 cursor-pointer hover:text-[#C3D4DE] transition-colors">
            <button class="flex items-center justify-between w-full gap-2" onclick={() => handleSort(col)}>
              <span>{col === '_id' ? '_id' : col}</span>
              {#if sortField === col}
                <span class="text-[#00ED64] text-[9px]">{sortDir === 'asc' ? '↑' : '↓'}</span>
              {/if}
            </button>
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
