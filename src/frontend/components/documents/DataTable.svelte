<script lang="ts">
  import {
    createTable,
    getCoreRowModel,
    getSortedRowModel,
    type SortingState,
    type ColumnDef,
  } from '@tanstack/table-core';

  let {
    data = [],
    columns = [],
    onRowClick,
    onSort,
  }: {
    data: any[];
    columns: string[];
    onRowClick?: (row: any, index: number) => void;
    onSort?: (field: string, dir: 'asc' | 'desc' | null) => void;
  } = $props();

  let sorting = $state<SortingState>([]);

  const tableColumns: ColumnDef<any, any>[] = columns.map(col => ({
    accessorKey: col,
    header: () => col === '_id' ? '_id' : col,
    cell: (info: any) => {
      const val = info.getValue();
      if (val === null || val === undefined) return 'null';
      if (typeof val === 'object' && val.$oid) return val.$oid;
      if (typeof val === 'object' && val.$date) {
        try { return new Date(val.$date.$numberLong || val.$date).toLocaleString(); } catch { return String(val.$date); }
      }
      if (Array.isArray(val)) return `[${val.length} items]`;
      if (typeof val === 'object') return '{...}';
      return String(val);
    },
  }));

  const table = createTable({
    get data() { return data; },
    columns: tableColumns,
    state: {
      get sorting() { return sorting; },
    },
    onSortingChange: (updater: any) => {
      const newSorting = typeof updater === 'function' ? updater(sorting) : updater;
      sorting = newSorting;
      if (newSorting.length > 0) {
        onSort?.(newSorting[0].id, newSorting[0].desc ? 'desc' : 'asc');
      } else {
        onSort?.('', null);
      }
    },
    getCoreRowModel: getCoreRowModel(),
    getSortedRowModel: getSortedRowModel(),
  });

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
      {#each table.getHeaderGroups() as headerGroup}
        <tr>
          {#each headerGroup.headers as header}
            <th class="border-b border-r border-[#2D3A45] px-3 py-2 text-left text-[10px] font-semibold text-[#7E97A7] whitespace-nowrap last:border-r-0 cursor-pointer hover:text-[#C3D4DE] transition-colors">
              <button class="flex items-center justify-between w-full gap-2" onclick={header.column.getToggleSortingHandler()}>
                <span>{header.column.columnDef.header}</span>
                {#if header.column.getIsSorted() === 'asc'}
                  <span class="text-[#00ED64] text-[9px]">↑</span>
                {:else if header.column.getIsSorted() === 'desc'}
                  <span class="text-[#00ED64] text-[9px]">↓</span>
                {/if}
              </button>
            </th>
          {/each}
        </tr>
      {/each}
    </thead>
    <tbody>
      {#each table.getRowModel().rows as row, idx}
        <tr class="cursor-pointer border-b border-[#1F2933] hover:bg-[#161D24] transition-colors" onclick={() => onRowClick?.(row.original, idx)}>
          {#each row.getVisibleCells() as cell}
            {@const val = cell.getValue()}
            <td class="border-b border-r border-[#1F2933] px-3 py-1.5 text-[11px] {getValueClass(val)} last:border-r-0">
              <span class="max-w-[200px] truncate block">{formatValue(val)}</span>
            </td>
          {/each}
        </tr>
      {/each}
    </tbody>
  </table>
</div>
