<script lang="ts">
  import { documentStore } from '../../stores/documentStore';

  let { connectionId, database, collection, onSelect }: {
    connectionId: string;
    database: string;
    collection: string;
    onSelect: (db: string, coll: string) => void;
  } = $props();

  let documents = $state<any[]>([]);
  let totalCount = $state(0);
  let page = $state(0);
  let loading = $state(false);
  let selectedDoc = $state<any>(null);
  let selectedId = $state<string | null>(null);
  let columns = $state<string[]>([]);

  const pageSize = 50;
  const totalPages = $derived(Math.ceil(totalCount / pageSize));

  async function loadDocs(p: number = 0) {
    loading = true;
    columns = [];
    selectedDoc = null;
    selectedId = null;
    await documentStore.loadDocuments(connectionId, database, collection, p);
    documents = [...documentStore.documents];
    totalCount = documentStore.totalCount;
    page = documentStore.page;
    if (documents.length > 0) {
      const allKeys = new Set<string>();
      allKeys.add('_id');
      documents.forEach(doc => {
        Object.keys(doc).forEach(k => allKeys.add(k));
      });
      columns = Array.from(allKeys);
    }
    loading = false;
  }

  $effect(() => {
    const _key = `${connectionId}-${database}-${collection}`;
    if (connectionId && database && collection) {
      loadDocs(0);
    } else {
      documents = [];
      columns = [];
      totalCount = 0;
    }
  });

  function selectDoc(doc: any) {
    selectedDoc = doc;
    selectedId = doc._id?.$oid || doc._id || null;
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

  function getValueType(val: any): string {
    if (val === null || val === undefined) return 'null';
    if (typeof val === 'object' && val.$oid) return 'objectId';
    if (typeof val === 'object' && val.$date) return 'date';
    if (typeof val === 'object' && Array.isArray(val)) return 'array';
    if (typeof val === 'object') return 'object';
    if (typeof val === 'number') return 'number';
    if (typeof val === 'boolean') return 'boolean';
    return 'string';
  }

  const typeColors: Record<string, string> = {
    string: 'text-emerald-400',
    number: 'text-cyan-400',
    objectId: 'text-amber-400',
    date: 'text-purple-400',
    boolean: 'text-orange-400',
    null: 'text-slate-400',
    array: 'text-slate-500',
    object: 'text-slate-500',
  };

  async function handleDelete() {
    if (selectedId && confirm('Delete this document?')) {
      await documentStore.deleteDocument(selectedId);
      selectedDoc = null;
      selectedId = null;
      await loadDocs(page);
    }
  }

  function goNext() {
    if (documents.length === pageSize) {
      loadDocs(page + 1);
    }
  }

  function goPrev() {
    if (page > 0) {
      loadDocs(page - 1);
    }
  }
</script>

<div class="flex h-full flex-col">
  <div class="flex items-center justify-between border-b border-[#2D3A45] px-4 py-2 bg-[#161D24]">
    <div class="flex items-center gap-2">
      <span class="text-[13px] font-medium text-[#C3D4DE]">{database}.{collection}</span>
      <span class="rounded bg-[#023430] px-1.5 py-0.5 text-[10px] font-medium text-[#00ED64]">{totalCount.toLocaleString()} docs</span>
    </div>
    <div class="flex items-center gap-2">
      <button class="rounded px-2 py-1 text-[11px] text-[#7E97A7] hover:bg-[#1F2933] hover:text-[#C3D4DE] transition-colors" onclick={() => loadDocs(page)}>
        <svg class="inline h-3 w-3 mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/></svg>
        Refresh
      </button>
      {#if selectedDoc}
        <button class="rounded px-2 py-1 text-[11px] text-red-400 hover:bg-red-500/10 transition-colors" onclick={handleDelete}>Delete</button>
      {/if}
    </div>
  </div>

  {#if loading}
    <div class="flex-1 flex items-center justify-center">
      <div class="text-center">
        <div class="mb-2 h-6 w-6 mx-auto animate-spin rounded-full border-2 border-[#00ED64] border-t-transparent"></div>
        <span class="text-[12px] text-[#7E97A7]">Loading documents...</span>
      </div>
    </div>
  {:else if documents.length === 0}
    <div class="flex-1 flex items-center justify-center">
      <div class="text-center">
        <svg class="mx-auto mb-3 h-12 w-12 text-[#2D3A45]" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1"><path stroke-linecap="round" stroke-linejoin="round" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4"/></svg>
        <span class="text-[13px] text-[#7E97A7]">No documents in this collection</span>
      </div>
    </div>
  {:else}
    <div class="flex-1 overflow-auto">
      <table class="w-full text-[12px] border-collapse">
        <thead class="sticky top-0 z-10 bg-[#0E1318]">
          <tr>
            {#each columns as col}
              <th class="px-3 py-2 text-left text-[10px] font-semibold uppercase tracking-wider text-[#7E97A7] border-b border-[#2D3A45] whitespace-nowrap">
                {#if col === '_id'}
                  <span class="text-[#FFC010]">_id</span>
                {:else}
                  {col}
                {/if}
              </th>
            {/each}
          </tr>
        </thead>
        <tbody>
          {#each documents as doc (JSON.stringify(doc._id))}
            <tr
              class="cursor-pointer border-b border-[#1F2933] transition-colors hover:bg-[#161D24] {selectedId === (doc._id?.$oid || doc._id) ? 'bg-[#023430]' : ''}"
              onclick={() => selectDoc(doc)}
            >
              {#each columns as col}
                {@const val = doc[col]}
                {@const type = getValueType(val)}
                <td class="px-3 py-1.5 font-mono text-[11px] {typeColors[type] || 'text-[#C3D4DE]'}">
                  <span class="max-w-[220px] truncate block">{formatValue(val)}</span>
                </td>
              {/each}
            </tr>
          {/each}
        </tbody>
      </table>
    </div>

    <div class="flex items-center justify-between border-t border-[#2D3A45] px-4 py-2 bg-[#0E1318]">
      <span class="text-[11px] text-[#7E97A7]">Page {page + 1} of {totalPages || 1} &middot; {totalCount.toLocaleString()} total</span>
      <div class="flex items-center gap-1">
        <button class="rounded px-3 py-1 text-[11px] text-[#7E97A7] hover:bg-[#1F2933] hover:text-[#C3D4DE] disabled:opacity-30 transition-colors" disabled={page === 0} onclick={goPrev}>Prev</button>
        <button class="rounded px-3 py-1 text-[11px] text-[#7E97A7] hover:bg-[#1F2933] hover:text-[#C3D4DE] disabled:opacity-30 transition-colors" disabled={documents.length < pageSize} onclick={goNext}>Next</button>
      </div>
    </div>
  {/if}

  {#if selectedDoc}
    <div class="border-t border-[#2D3A45] bg-[#0E1318] p-3 max-h-[200px] overflow-auto">
      <div class="mb-1 flex items-center justify-between">
        <span class="text-[10px] font-semibold uppercase tracking-wider text-[#7E97A7]">Document Preview</span>
        <button class="rounded px-2 py-0.5 text-[10px] text-[#7E97A7] hover:bg-[#1F2933]" onclick={() => navigator.clipboard.writeText(JSON.stringify(selectedDoc, null, 2))}>Copy JSON</button>
      </div>
      <pre class="text-[11px] font-mono text-[#C3D4DE] whitespace-pre-wrap leading-relaxed">{JSON.stringify(selectedDoc, null, 2)}</pre>
    </div>
  {/if}
</div>
