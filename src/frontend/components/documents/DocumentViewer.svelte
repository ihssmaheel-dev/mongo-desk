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
    await documentStore.loadDocuments(connectionId, database, collection, p);
    documents = [...documentStore.documents];
    totalCount = documentStore.totalCount;
    page = documentStore.page;
    if (documents.length > 0 && columns.length === 0) {
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
    if (connectionId && database && collection) {
      loadDocs(0);
    }
  });

  function selectDoc(doc: any) {
    selectedDoc = doc;
    selectedId = doc._id?.$oid || doc._id || null;
  }

  function formatValue(val: any): string {
    if (val === null || val === undefined) return 'null';
    if (typeof val === 'object' && val.$oid) return val.$oid;
    if (typeof val === 'object' && val.$date) return new Date(val.$date).toLocaleString();
    if (typeof val === 'object') return JSON.stringify(val);
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
    array: 'text-slate-400',
    object: 'text-slate-400',
  };

  async function handleDelete() {
    if (selectedId && confirm('Delete this document?')) {
      await documentStore.deleteDocument(selectedId);
      selectedDoc = null;
      selectedId = null;
      await loadDocs(page);
    }
  }
</script>

<div class="flex h-full flex-col">
  <div class="flex items-center justify-between border-b border-[var(--border-subtle)] px-3 py-2">
    <div class="flex items-center gap-2">
      <span class="text-[12px] font-medium text-[var(--text-primary)]">{database}.{collection}</span>
      <span class="text-[11px] text-[var(--text-tertiary)]">{totalCount.toLocaleString()} docs</span>
    </div>
    <div class="flex items-center gap-2">
      <button class="rounded px-2 py-1 text-[11px] text-[var(--text-tertiary)] hover:bg-[var(--bg-surface)]" onclick={() => loadDocs(page)}>Refresh</button>
      {#if selectedDoc}
        <button class="rounded px-2 py-1 text-[11px] text-red-400 hover:bg-red-500/10" onclick={handleDelete}>Delete</button>
      {/if}
    </div>
  </div>

  {#if loading}
    <div class="flex-1 flex items-center justify-center">
      <span class="text-[12px] text-[var(--text-tertiary)]">Loading documents...</span>
    </div>
  {:else if documents.length === 0}
    <div class="flex-1 flex items-center justify-center">
      <span class="text-[12px] text-[var(--text-tertiary)]">No documents found</span>
    </div>
  {:else}
    <div class="flex-1 overflow-auto">
      <table class="w-full text-[12px]">
        <thead class="sticky top-0 bg-[var(--bg-panel)]">
          <tr class="border-b border-[var(--border-subtle)]">
            {#each columns as col}
              <th class="px-3 py-1.5 text-left font-medium text-[var(--text-tertiary)]">{col}</th>
            {/each}
          </tr>
        </thead>
        <tbody>
          {#each documents as doc (doc._id?.$oid || doc._id)}
            <tr
              class="cursor-pointer border-b border-[var(--border-subtle)] hover:bg-[var(--bg-surface)] {selectedId === (doc._id?.$oid || doc._id) ? 'bg-[var(--accent-primary-subtle)]' : ''}"
              onclick={() => selectDoc(doc)}
            >
              {#each columns as col}
                {@const val = doc[col]}
                {@const type = getValueType(val)}
                <td class="px-3 py-1.5 font-mono text-[11px] {typeColors[type] || 'text-[var(--text-secondary)]'}">
                  <span class="max-w-[200px] truncate block">{formatValue(val)}</span>
                </td>
              {/each}
            </tr>
          {/each}
        </tbody>
      </table>
    </div>

    <div class="flex items-center justify-between border-t border-[var(--border-subtle)] px-3 py-2">
      <span class="text-[11px] text-[var(--text-tertiary)]">Page {page + 1} of {totalPages || 1}</span>
      <div class="flex items-center gap-1">
        <button class="rounded px-2 py-1 text-[11px] text-[var(--text-tertiary)] hover:bg-[var(--bg-surface)] disabled:opacity-50" disabled={page === 0} onclick={() => documentStore.prevPage()}>Prev</button>
        <button class="rounded px-2 py-1 text-[11px] text-[var(--text-tertiary)] hover:bg-[var(--bg-surface)] disabled:opacity-50" disabled={!documentStore.documents.length || documentStore.documents.length < pageSize} onclick={() => documentStore.nextPage()}>Next</button>
      </div>
    </div>
  {/if}

  {#if selectedDoc}
    <div class="border-t border-[var(--border-subtle)] bg-[var(--bg-panel)] p-3 max-h-[200px] overflow-auto">
      <div class="mb-1 text-[10px] font-semibold uppercase tracking-wider text-[var(--text-tertiary)]">Document Preview</div>
      <pre class="text-[11px] font-mono text-[var(--text-secondary)] whitespace-pre-wrap">{JSON.stringify(selectedDoc, null, 2)}</pre>
    </div>
  {/if}
</div>
