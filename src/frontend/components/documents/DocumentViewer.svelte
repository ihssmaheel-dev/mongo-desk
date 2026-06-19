<script lang="ts">
  import { documentStore } from '../../stores/documentStore';
  import ConfirmDialog from '../common/ConfirmDialog.svelte';

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
  let selectedIdx = $state<number>(-1);
  let columns = $state<string[]>([]);
  let showDeleteConfirm = $state(false);
  let viewMode = $state<'table' | 'json'>('table');
  let sortField = $state('');
  let sortDir = $state<'asc' | 'desc'>('asc');
  let showInsertDialog = $state(false);
  let showEditDialog = $state(false);
  let insertJson = $state('{\n  \n}');
  let editJson = $state('');
  let showFullPreview = $state(false);
  let filters = $state<Record<string, { op: string; value: string }>>({});
  let activeFilterCol = $state('');
  let filterOp = $state('eq');
  let filterValue = $state('');

  const pageSize = 50;
  const totalPages = $derived(Math.ceil(totalCount / pageSize));

  function detectType(val: string): any {
    if (val === 'null') return null;
    if (val === 'true') return true;
    if (val === 'false') return false;
    if (/^-?\d+$/.test(val)) return parseInt(val);
    if (/^-?\d+\.\d+$/.test(val)) return parseFloat(val);
    if (/^\d{4}-\d{2}-\d{2}/.test(val)) return { $date: val };
    return val;
  }

  function getFieldType(col: string): string {
    if (documents.length === 0) return 'string';
    for (const doc of documents) {
      const val = doc[col];
      if (val === null || val === undefined) continue;
      if (typeof val === 'number') return 'number';
      if (typeof val === 'boolean') return 'boolean';
      if (typeof val === 'object' && val.$oid) return 'objectId';
      if (typeof val === 'object' && val.$date) return 'date';
      if (Array.isArray(val)) return 'array';
      if (typeof val === 'object') return 'object';
    }
    return 'string';
  }

  function getFilterOps(fieldType: string): { value: string; label: string }[] {
    switch (fieldType) {
      case 'number': return [
        { value: 'eq', label: '= Equals' }, { value: 'ne', label: '≠ Not equals' },
        { value: 'gt', label: '> Greater than' }, { value: 'gte', label: '≥ Greater or equal' },
        { value: 'lt', label: '< Less than' }, { value: 'lte', label: '≤ Less or equal' },
      ];
      case 'date': return [
        { value: 'eq', label: '= Equals' }, { value: 'gt', label: '> After' },
        { value: 'gte', label: '≥ After or equal' }, { value: 'lt', label: '< Before' },
        { value: 'lte', label: '≤ Before or equal' },
      ];
      case 'objectId': return [
        { value: 'eq', label: '= Equals' },
      ];
      default: return [
        { value: 'eq', label: '= Equals' }, { value: 'ne', label: '≠ Not equals' },
        { value: 'contains', label: 'Contains' }, { value: 'regex', label: 'Regex' },
      ];
    }
  }

  function buildFilterQuery(): string | undefined {
    const obj: any = {};
    for (const [col, f] of Object.entries(filters)) {
      if (!f.value) continue;
      const fieldType = getFieldType(col);
      let val: any;
      if (fieldType === 'objectId') {
        val = { $oid: f.value };
      } else if (fieldType === 'number') {
        val = detectType(f.value);
      } else if (fieldType === 'date') {
        val = { $date: f.value };
      } else {
        val = detectType(f.value);
      }
      switch (f.op) {
        case 'eq': obj[col] = val; break;
        case 'ne': obj[col] = { $ne: val }; break;
        case 'gt': obj[col] = { $gt: val }; break;
        case 'gte': obj[col] = { $gte: val }; break;
        case 'lt': obj[col] = { $lt: val }; break;
        case 'lte': obj[col] = { $lte: val }; break;
        case 'contains': obj[col] = { $regex: f.value, $options: 'i' }; break;
        case 'regex': obj[col] = { $regex: f.value }; break;
      }
    }
    return Object.keys(obj).length > 0 ? JSON.stringify(obj) : undefined;
  }

  function buildSortQuery(): string | undefined {
    if (!sortField) return undefined;
    return JSON.stringify({ [sortField]: sortDir === 'asc' ? 1 : -1 });
  }

  async function executeQuery(p?: number) {
    loading = true; columns = []; selectedDoc = null; selectedIdx = -1;
    const pageNum = p !== undefined ? p : page;
    await documentStore.loadDocuments(connectionId, database, collection, pageNum, buildFilterQuery(), buildSortQuery());
    documents = [...documentStore.documents];
    totalCount = documentStore.totalCount;
    page = documentStore.page;
    if (documents.length > 0) {
      const allKeys = new Set<string>(); allKeys.add('_id');
      documents.forEach(doc => Object.keys(doc).forEach(k => allKeys.add(k)));
      columns = Array.from(allKeys);
    }
    loading = false;
  }

  $effect(() => {
    const _key = `${connectionId}-${database}-${collection}`;
    if (connectionId && database && collection) executeQuery(0);
    else { documents = []; columns = []; totalCount = 0; }
  });

  function handleSort(field: string) {
    if (sortField === field) {
      if (sortDir === 'asc') sortDir = 'desc';
      else { sortField = ''; sortDir = 'asc'; }
    } else { sortField = field; sortDir = 'asc'; }
    executeQuery(0);
  }

  function openFilter(col: string) {
    activeFilterCol = activeFilterCol === col ? '' : col;
    if (filters[col]) { filterOp = filters[col].op; filterValue = filters[col].value; }
    else { filterOp = 'eq'; filterValue = ''; }
  }

  function applyFilter() {
    if (!filterValue) { clearFilter(activeFilterCol); return; }
    filters = { ...filters, [activeFilterCol]: { op: filterOp, value: filterValue } };
    activeFilterCol = '';
    executeQuery(0);
  }

  function clearFilter(col: string) {
    const n = { ...filters }; delete n[col]; filters = n;
    activeFilterCol = '';
    executeQuery(0);
  }

  function clearAllFilters() {
    filters = {}; sortField = ''; sortDir = 'asc';
    executeQuery(0);
  }

  function selectDoc(doc: any, idx: number) { selectedDoc = doc; selectedIdx = idx; }

  function formatValue(val: any): string {
    if (val === null || val === undefined) return 'null';
    if (typeof val === 'object' && val.$oid) return val.$oid;
    if (typeof val === 'object' && val.$date) { try { return new Date(val.$date.$numberLong || val.$date).toLocaleString(); } catch { return String(val.$date); } }
    if (typeof val === 'object' && Array.isArray(val)) return `[${val.length} items]`;
    if (typeof val === 'object') return '{...}';
    return String(val);
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

  async function handleDelete() {
    if (selectedDoc) {
      const id = selectedDoc._id?.$oid || selectedDoc._id;
      if (id) { await documentStore.deleteDocument(id); selectedDoc = null; selectedIdx = -1; await executeQuery(); }
    }
  }

  async function handleInsert() {
    try { JSON.parse(insertJson); const ok = await documentStore.insertDocument(insertJson); if (ok) { showInsertDialog = false; insertJson = '{\n  \n}'; await executeQuery(0); } }
    catch (e) { alert('Invalid JSON: ' + (e as Error).message); }
  }

  function openEdit() { if (selectedDoc) { editJson = JSON.stringify(selectedDoc, null, 2); showEditDialog = true; } }

  async function handleEdit() {
    try {
      const parsed = JSON.parse(editJson); const id = selectedDoc._id?.$oid || selectedDoc._id;
      if (id) { const { _id, ...updateData } = parsed; const ok = await documentStore.updateDocument(id, JSON.stringify(updateData));
        if (ok) { showEditDialog = false; selectedDoc = null; selectedIdx = -1; await executeQuery(); } }
    } catch (e) { alert('Invalid JSON: ' + (e as Error).message); }
  }

  function highlightJson(obj: any, indent: number = 0): string {
    if (obj === null || obj === undefined) return '<span style="color:#7E97A7">null</span>';
    if (typeof obj === 'boolean') return `<span style="color:#FF8966">${obj}</span>`;
    if (typeof obj === 'number') return `<span style="color:#5DD0FF">${obj}</span>`;
    if (typeof obj === 'string') return `<span style="color:#00ED64">"${obj}"</span>`;
    if (Array.isArray(obj)) { if (obj.length === 0) return '<span style="color:#C3D4DE">[]</span>'; const p = '  '.repeat(indent+1); return `<span style="color:#C3D4DE">[</span>\n${obj.map(i => p+highlightJson(i,indent+1)).join(',\n')}\n${'  '.repeat(indent)}<span style="color:#C3D4DE">]</span>`; }
    if (typeof obj === 'object') { const k = Object.keys(obj); if (!k.length) return '<span style="color:#C3D4DE">{}</span>'; const p = '  '.repeat(indent+1); return `<span style="color:#C3D4DE">{</span>\n${k.map(key => `${p}<span style="color:#5DD0FF">"${key}"</span><span style="color:#C3D4DE">: </span>${highlightJson(obj[key],indent+1)}`).join(',\n')}\n${'  '.repeat(indent)}<span style="color:#C3D4DE">}</span>`; }
    return String(obj);
  }

  function goNext() { if (documents.length >= pageSize) executeQuery(page + 1); }
  function goPrev() { if (page > 0) executeQuery(page - 1); }
</script>

{#if showFullPreview && selectedDoc}
  <div class="fixed inset-0 z-50 flex flex-col bg-[#0E1318]">
    <div class="flex items-center justify-between border-b border-[#2D3A45] px-4 py-2 bg-[#161D24]">
      <span class="text-[13px] font-semibold text-[#C3D4DE]">Document Preview</span>
      <div class="flex items-center gap-2">
        <button class="rounded px-2 py-1 text-[11px] text-[#7E97A7] hover:bg-[#1F2933] hover:text-[#C3D4DE]" onclick={() => navigator.clipboard.writeText(JSON.stringify(selectedDoc, null, 2))}>Copy JSON</button>
        <button class="rounded px-2 py-1 text-[11px] text-[#7E97A7] hover:bg-[#1F2933] hover:text-[#C3D4DE]" onclick={() => { editJson = JSON.stringify(selectedDoc, null, 2); showEditDialog = true; showFullPreview = false; }}>Edit</button>
        <button class="rounded p-1 text-[#465A6B] hover:bg-[#2D3A45] hover:text-[#C3D4DE]" onclick={() => showFullPreview = false}><svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" d="M6 18L18 6M6 6l12 12"/></svg></button>
      </div>
    </div>
    <div class="flex-1 overflow-auto p-4"><pre class="text-[12px] font-mono leading-relaxed whitespace-pre-wrap">{@html highlightJson(selectedDoc)}</pre></div>
  </div>
{/if}

<div class="flex h-full flex-col bg-[#0E1318]">
  <div class="flex items-center justify-between border-b border-[#2D3A45] px-4 py-2 bg-[#161D24]">
    <div class="flex items-center gap-3">
      <span class="text-[13px] font-semibold text-[#C3D4DE]">{collection}</span>
      <span class="rounded-full bg-[#023430] px-2 py-0.5 text-[10px] font-medium text-[#00ED64]">{totalCount.toLocaleString()} docs</span>
      <span class="text-[11px] text-[#7E97A7]">{database}</span>
    </div>
    <div class="flex items-center gap-2">
      <div class="flex rounded border border-[#2D3A45]">
        <button class="px-2 py-1 text-[11px] {viewMode === 'table' ? 'bg-[#023430] text-[#00ED64]' : 'text-[#7E97A7] hover:text-[#C3D4DE]'}" onclick={() => viewMode = 'table'}>Table</button>
        <button class="px-2 py-1 text-[11px] {viewMode === 'json' ? 'bg-[#023430] text-[#00ED64]' : 'text-[#7E97A7] hover:text-[#C3D4DE]'}" onclick={() => viewMode = 'json'}>JSON</button>
      </div>
      <button class="flex items-center gap-1 rounded bg-[#00684A] px-2 py-1 text-[11px] text-white hover:bg-[#00C75A]" onclick={() => showInsertDialog = true}><svg class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" d="M12 4v16m8-8H4"/></svg>Insert</button>
      {#if selectedDoc}
        <button class="rounded border border-[#2D3A45] px-2 py-1 text-[11px] text-[#7E97A7] hover:bg-[#1F2933] hover:text-[#C3D4DE]" onclick={openEdit}>Edit</button>
        <button class="rounded px-2 py-1 text-[11px] text-[#FF5C5C] hover:bg-[#FF5C5C]/10" onclick={() => showDeleteConfirm = true}><svg class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"/></svg>Delete</button>
      {/if}
      <button class="rounded px-2 py-1 text-[11px] text-[#7E97A7] hover:bg-[#1F2933] hover:text-[#C3D4DE]" onclick={() => executeQuery(0)}><svg class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/></svg>Refresh</button>
    </div>
  </div>

  {#if viewMode === 'table'}
    <div class="flex items-center gap-2 border-b border-[#2D3A45] px-4 py-1.5 bg-[#161D24]">
      <span class="text-[10px] text-[#465A6B]">Query:</span>
      <code class="flex-1 truncate font-mono text-[10px] text-[#00ED64]">{JSON.stringify({ ...(buildFilterQuery() ? { filter: JSON.parse(buildFilterQuery()!) } : {}), ...(buildSortQuery() ? { sort: JSON.parse(buildSortQuery()!) } : {}) }, null, 2)}</code>
      {#if Object.keys(filters).length > 0 || sortField}
        <button class="text-[10px] text-[#FF5C5C] hover:text-[#FF8966]" onclick={clearAllFilters}>Clear all</button>
      {/if}
    </div>
  {/if}

  {#if loading}
    <div class="flex-1 flex items-center justify-center">
      <div class="text-center"><div class="mb-3 mx-auto h-8 w-8 animate-spin rounded-full border-2 border-[#00ED64] border-t-transparent"></div><span class="text-[12px] text-[#7E97A7]">Loading documents...</span></div>
    </div>
  {:else if documents.length === 0}
    <div class="flex-1 flex items-center justify-center">
      <div class="text-center"><svg class="mx-auto mb-3 h-12 w-12 text-[#2D3A45]" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1"><path stroke-linecap="round" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4"/></svg><p class="text-[13px] text-[#7E97A7]">No documents found</p></div>
    </div>
  {:else if viewMode === 'table'}
    <div class="flex-1 overflow-auto">
      <table class="w-full border-collapse">
        <thead class="sticky top-0 z-10"><tr class="bg-[#0E1318]">
          {#each columns as col}
            <th class="border-b border-[#2D3A45] px-3 py-2 text-left text-[10px] font-semibold text-[#7E97A7] whitespace-nowrap group">
              <div class="flex items-center gap-1">
                <button class="hover:text-[#C3D4DE] transition-colors {sortField === col ? 'text-[#00ED64]' : ''}" onclick={() => handleSort(col)}>
                  {#if col === '_id'}<span class="text-[#FFC010]">_id</span>{:else}{col}{/if}
                  {#if sortField === col}<span class="ml-0.5">{sortDir === 'asc' ? '↑' : '↓'}</span>{/if}
                </button>
                <button class="opacity-0 group-hover:opacity-100 {filters[col] ? 'opacity-100 !text-[#00ED64]' : 'text-[#465A6B]'} hover:text-[#C3D4DE] transition-all" onclick={() => openFilter(col)}>
                  <svg class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" d="M3 4a1 1 0 011-1h16a1 1 0 011 1v2.586a1 1 0 01-.293.707l-6.414 6.414a1 1 0 00-.293.707V17l-4 4v-6.586a1 1 0 00-.293-.707L3.293 7.293A1 1 0 013 6.586V4z"/></svg>
                </button>
              </div>
            </th>
          {/each}
        </tr></thead>
        <tbody>
          {#each documents as doc, idx (JSON.stringify(doc._id))}
            <tr class="cursor-pointer border-b border-[#1F2933] hover:bg-[#161D24] {selectedIdx === idx ? 'bg-[#023430]' : ''}" onclick={() => selectDoc(doc, idx)}>
              {#each columns as col}<td class="border-b border-[#1F2933] px-3 py-1.5 text-[11px] {getValueClass(doc[col])}"><span class="max-w-[200px] truncate block">{formatValue(doc[col])}</span></td>{/each}
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {:else}
    <div class="flex-1 overflow-auto p-4"><pre class="text-[11px] font-mono text-[#C3D4DE] whitespace-pre-wrap">{JSON.stringify(documents, null, 2)}</pre></div>
  {/if}

  <div class="flex items-center justify-between border-t border-[#2D3A45] px-4 py-2 bg-[#161D24]">
    <span class="text-[11px] text-[#7E97A7]">Page {page + 1} of {totalPages || 1} · {totalCount.toLocaleString()} total</span>
    <div class="flex items-center gap-1">
      <button class="rounded px-3 py-1 text-[11px] text-[#7E97A7] hover:bg-[#1F2933] hover:text-[#C3D4DE] disabled:opacity-30" disabled={page === 0} onclick={goPrev}>Prev</button>
      <button class="rounded px-3 py-1 text-[11px] text-[#7E97A7] hover:bg-[#1F2933] hover:text-[#C3D4DE] disabled:opacity-30" disabled={documents.length < pageSize} onclick={goNext}>Next</button>
    </div>
  </div>

  {#if selectedDoc && viewMode === 'table' && !showFullPreview}
    <div class="flex flex-col border-t border-[#2D3A45] bg-[#161D24] h-[250px]">
      <div class="flex items-center justify-between px-4 py-1.5 border-b border-[#2D3A45] flex-shrink-0">
        <span class="text-[10px] font-semibold uppercase tracking-wider text-[#7E97A7]">Document Preview</span>
        <div class="flex items-center gap-1">
          <button class="rounded px-2 py-0.5 text-[10px] text-[#7E97A7] hover:bg-[#1F2933] hover:text-[#C3D4DE]" onclick={() => showFullPreview = true}>Full Preview</button>
          <button class="rounded px-2 py-0.5 text-[10px] text-[#7E97A7] hover:bg-[#1F2933] hover:text-[#C3D4DE]" onclick={() => navigator.clipboard.writeText(JSON.stringify(selectedDoc, null, 2))}>Copy JSON</button>
          <button class="rounded p-0.5 text-[#465A6B] hover:bg-[#2D3A45] hover:text-[#C3D4DE]" onclick={() => { selectedDoc = null; selectedIdx = -1; }}><svg class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" d="M6 18L18 6M6 6l12 12"/></svg></button>
        </div>
      </div>
      <div class="flex-1 overflow-auto p-3"><pre class="text-[11px] font-mono leading-relaxed whitespace-pre-wrap">{@html highlightJson(selectedDoc)}</pre></div>
    </div>
  {/if}
</div>

<ConfirmDialog bind:open={showDeleteConfirm} title="Delete Document" message="Are you sure you want to delete this document?" confirmText="Delete" variant="danger" onConfirm={handleDelete} />

{#if activeFilterCol}
  <div class="fixed inset-0 z-50 flex items-end justify-center pb-20" role="dialog">
    <button class="fixed inset-0 bg-black/40" onclick={() => activeFilterCol = ''}></button>
    <div class="relative w-full max-w-sm rounded-xl border border-[#2D3A45] bg-[#1F2933] shadow-2xl p-4" onclick={(e) => e.stopPropagation()}>
      <div class="mb-3 flex items-center justify-between">
        <div>
          <div class="text-[13px] font-semibold text-[#C3D4DE]">{activeFilterCol}</div>
          <div class="text-[10px] text-[#465A6B]">{getFieldType(activeFilterCol)} field</div>
        </div>
        <button class="rounded p-1 text-[#465A6B] hover:bg-[#2D3A45] hover:text-[#C3D4DE] transition-colors" onclick={() => activeFilterCol = ''}>
          <svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" d="M6 18L18 6M6 6l12 12"/></svg>
        </button>
      </div>

      <div class="mb-3">
        <label class="mb-1 block text-[10px] text-[#7E97A7]">Operator</label>
        <select class="w-full rounded border border-[#2D3A45] bg-[#0E1318] px-3 py-2 text-[12px] text-[#C3D4DE] outline-none focus:border-[#00ED64] transition-colors" bind:value={filterOp}>
          {#each getFilterOps(getFieldType(activeFilterCol)) as op}
            <option value={op.value}>{op.label}</option>
          {/each}
        </select>
      </div>

      <div class="mb-4">
        <label class="mb-1 block text-[10px] text-[#7E97A7]">Value</label>
        {#if getFieldType(activeFilterCol) === 'number'}
          <input type="number" step="any" bind:value={filterValue} placeholder="Enter number..."
            class="w-full rounded border border-[#2D3A45] bg-[#0E1318] px-3 py-2 font-mono text-[12px] text-[#C3D4DE] placeholder-[#465A6B] outline-none focus:border-[#00ED64] transition-colors"
            onkeydown={(e) => { if (e.key === 'Enter') applyFilter(); }} />
        {:else if getFieldType(activeFilterCol) === 'date'}
          <input type="datetime-local" bind:value={filterValue}
            class="w-full rounded border border-[#2D3A45] bg-[#0E1318] px-3 py-2 font-mono text-[12px] text-[#C3D4DE] outline-none focus:border-[#00ED64] transition-colors"
            onkeydown={(e) => { if (e.key === 'Enter') applyFilter(); }} />
        {:else if getFieldType(activeFilterCol) === 'boolean'}
          <select class="w-full rounded border border-[#2D3A45] bg-[#0E1318] px-3 py-2 text-[12px] text-[#C3D4DE] outline-none focus:border-[#00ED64] transition-colors" bind:value={filterValue}>
            <option value="true">true</option>
            <option value="false">false</option>
          </select>
        {:else if getFieldType(activeFilterCol) === 'objectId'}
          <input type="text" bind:value={filterValue} placeholder="Paste ObjectId..."
            class="w-full rounded border border-[#2D3A45] bg-[#0E1318] px-3 py-2 font-mono text-[12px] text-[#C3D4DE] placeholder-[#465A6B] outline-none focus:border-[#00ED64] transition-colors"
            onkeydown={(e) => { if (e.key === 'Enter') applyFilter(); }} />
        {:else}
          <input type="text" bind:value={filterValue} placeholder="Enter value..."
            class="w-full rounded border border-[#2D3A45] bg-[#0E1318] px-3 py-2 font-mono text-[12px] text-[#C3D4DE] placeholder-[#465A6B] outline-none focus:border-[#00ED64] transition-colors"
            onkeydown={(e) => { if (e.key === 'Enter') applyFilter(); }} />
        {/if}
      </div>

      <div class="flex gap-2">
        {#if filters[activeFilterCol]}
          <button class="rounded-lg border border-[#2D3A45] px-4 py-2 text-[12px] text-[#FF5C5C] hover:bg-[#FF5C5C]/10 transition-colors" onclick={() => clearFilter(activeFilterCol)}>Clear Filter</button>
        {/if}
        <button class="flex-1 rounded-lg bg-[#00684A] px-4 py-2 text-[12px] font-medium text-white hover:bg-[#00C75A] transition-colors" onclick={applyFilter}>Apply Filter</button>
      </div>
    </div>
  </div>
{/if}

{#if showInsertDialog}
  <div class="fixed inset-0 z-50 flex items-center justify-center" role="dialog">
    <button class="fixed inset-0 bg-black/60 backdrop-blur-sm" onclick={() => showInsertDialog = false}></button>
    <div class="relative w-full max-w-lg rounded-lg bg-[#1F2933] shadow-2xl border border-[#2D3A45]">
      <div class="flex items-center justify-between border-b border-[#2D3A45] px-5 py-3"><h2 class="text-[14px] font-semibold text-[#C3D4DE]">Insert Document</h2><button class="rounded p-1 text-[#465A6B] hover:bg-[#2D3A45] hover:text-[#C3D4DE]" onclick={() => showInsertDialog = false}><svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" d="M6 18L18 6M6 6l12 12"/></svg></button></div>
      <div class="px-5 py-4"><textarea bind:value={insertJson} rows="12" class="w-full rounded border border-[#2D3A45] bg-[#0E1318] p-3 font-mono text-[12px] text-[#C3D4DE] outline-none focus:border-[#00ED64] resize-y"></textarea></div>
      <div class="flex justify-end gap-2 border-t border-[#2D3A45] px-5 py-3"><button class="rounded border border-[#2D3A45] px-4 py-1.5 text-[12px] text-[#7E97A7] hover:bg-[#2D3A45] hover:text-[#C3D4DE]" onclick={() => showInsertDialog = false}>Cancel</button><button class="rounded bg-[#00684A] px-4 py-1.5 text-[12px] font-medium text-white hover:bg-[#00C75A]" onclick={handleInsert}>Insert</button></div>
    </div>
  </div>
{/if}

{#if showEditDialog}
  <div class="fixed inset-0 z-50 flex items-center justify-center" role="dialog">
    <button class="fixed inset-0 bg-black/60 backdrop-blur-sm" onclick={() => showEditDialog = false}></button>
    <div class="relative w-full max-w-lg rounded-lg bg-[#1F2933] shadow-2xl border border-[#2D3A45]">
      <div class="flex items-center justify-between border-b border-[#2D3A45] px-5 py-3"><h2 class="text-[14px] font-semibold text-[#C3D4DE]">Edit Document</h2><button class="rounded p-1 text-[#465A6B] hover:bg-[#2D3A45] hover:text-[#C3D4DE]" onclick={() => showEditDialog = false}><svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" d="M6 18L18 6M6 6l12 12"/></svg></button></div>
      <div class="px-5 py-4"><textarea bind:value={editJson} rows="12" class="w-full rounded border border-[#2D3A45] bg-[#0E1318] p-3 font-mono text-[12px] text-[#C3D4DE] outline-none focus:border-[#00ED64] resize-y"></textarea></div>
      <div class="flex justify-end gap-2 border-t border-[#2D3A45] px-5 py-3"><button class="rounded border border-[#2D3A45] px-4 py-1.5 text-[12px] text-[#7E97A7] hover:bg-[#2D3A45] hover:text-[#C3D4DE]" onclick={() => showEditDialog = false}>Cancel</button><button class="rounded bg-[#00684A] px-4 py-1.5 text-[12px] font-medium text-white hover:bg-[#00C75A]" onclick={handleEdit}>Save</button></div>
    </div>
  </div>
{/if}
