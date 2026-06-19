<script lang="ts">
  import { documentStore } from '../../stores/documentStore';
  import ConfirmDialog from '../common/ConfirmDialog.svelte';
  import FilterDialog from '../common/FilterDialog.svelte';
  import DataTable from './DataTable.svelte';

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
  let filterDialogOpen = $state(false);
  let filterDialogCol = $state('');
  let filterDialogOp = $state('eq');
  let filterDialogValue = $state('');
  let lastLoadKey = '';
  let queryId = $state(0);
  let fieldTypes = $state<Record<string, string>>({});

  const pageSize = 50;
  const totalPages = $derived(Math.ceil(totalCount / pageSize));

  function getFieldType(col: string): string {
    if (fieldTypes[col]) return fieldTypes[col];
    if (documents.length === 0) return 'string';
    for (const doc of documents) {
      const val = doc[col];
      if (val === null || val === undefined) continue;
      let t = 'string';
      if (typeof val === 'number') t = 'number';
      else if (typeof val === 'boolean') t = 'boolean';
      else if (typeof val === 'object' && val.$oid) t = 'objectId';
      else if (typeof val === 'object' && val.$date) t = 'date';
      else if (Array.isArray(val)) t = 'array';
      else if (typeof val === 'object') t = 'object';
      fieldTypes[col] = t;
      return t;
    }
    return 'string';
  }

  function buildFilterQuery(): string | undefined {
    const obj: any = {};
    for (const [col, f] of Object.entries(filters)) {
      if (!f.value && !['isTrue','isFalse','exists','notExists','today','thisWeek','thisMonth','empty','notEmpty','isEmpty','notHasKey'].includes(f.op)) continue;
      const ft = getFieldType(col);
      switch (f.op) {
        case 'eq':
          if (ft === 'objectId') obj[col] = { $oid: f.value };
          else if (ft === 'date') obj[col] = { $date: f.value };
          else if (ft === 'array') obj[col] = f.value;
          else obj[col] = detectType(f.value);
          break;
        case 'ne':
          if (ft === 'objectId') obj[col] = { $ne: { $oid: f.value } };
          else if (ft === 'date') obj[col] = { $ne: { $date: f.value } };
          else obj[col] = { $ne: detectType(f.value) };
          break;
        case 'gt': obj[col] = { $gt: detectType(f.value) }; break;
        case 'gte': obj[col] = { $gte: detectType(f.value) }; break;
        case 'lt': obj[col] = { $lt: detectType(f.value) }; break;
        case 'lte': obj[col] = { $lte: detectType(f.value) }; break;
        case 'between': { const [a, b] = f.value.split(',').map(s => detectType(s.trim())); obj[col] = { $gte: a, $lte: b }; break; }
        case 'contains':
          if (ft === 'array') obj[col] = f.value;
          else obj[col] = { $regex: f.value, $options: 'i' };
          break;
        case 'startsWith': obj[col] = { $regex: `^${f.value}`, $options: 'i' }; break;
        case 'endsWith': obj[col] = { $regex: `${f.value}$`, $options: 'i' }; break;
        case 'regex': obj[col] = { $regex: f.value }; break;
        case 'isTrue': obj[col] = true; break;
        case 'isFalse': obj[col] = false; break;
        case 'exists': obj[col] = { $exists: true }; break;
        case 'notExists': obj[col] = { $exists: false }; break;
        case 'today': { const d = new Date(); d.setHours(0,0,0,0); const d2 = new Date(); d2.setHours(23,59,59,999); obj[col] = { $gte: { $date: d.toISOString() }, $lte: { $date: d2.toISOString() } }; break; }
        case 'thisWeek': { const now = new Date(); const start = new Date(now); start.setDate(now.getDate() - now.getDay()); start.setHours(0,0,0,0); const end = new Date(start); end.setDate(start.getDate() + 7); obj[col] = { $gte: { $date: start.toISOString() }, $lt: { $date: end.toISOString() } }; break; }
        case 'thisMonth': { const now = new Date(); const start = new Date(now.getFullYear(), now.getMonth(), 1); const end = new Date(now.getFullYear(), now.getMonth() + 1, 0, 23, 59, 59, 999); obj[col] = { $gte: { $date: start.toISOString() }, $lte: { $date: end.toISOString() } }; break; }
        case 'notContains':
          if (ft === 'array') obj[col] = { $not: { $elemMatch: { $eq: f.value } } };
          else obj[col] = { $not: { $regex: f.value, $options: 'i' } };
          break;
        case 'sizeEquals': obj[col] = { $size: parseInt(f.value) }; break;
        case 'sizeGt': obj[col] = { $exists: true, $not: { $size: { $lte: parseInt(f.value) } } }; break;
        case 'sizeLt': obj[col] = { $exists: true, $not: { $size: { $gte: parseInt(f.value) } } }; break;
        case 'empty': obj[col] = { $size: 0 }; break;
        case 'notEmpty': obj[col] = { $exists: true, $not: { $size: 0 } }; break;
        case 'hasKey': obj[`${col}.${f.value}`] = { $exists: true }; break;
        case 'notHasKey': obj[`${col}.${f.value}`] = { $exists: false }; break;
        case 'isEmpty': obj[col] = { $eq: {} }; break;
      }
    }
    return Object.keys(obj).length > 0 ? JSON.stringify(obj) : undefined;
  }

  function detectType(val: string): any {
    if (val === 'null') return null;
    if (val === 'true') return true;
    if (val === 'false') return false;
    if (/^-?\d+$/.test(val)) return parseInt(val);
    if (/^-?\d+\.\d+$/.test(val)) return parseFloat(val);
    if (/^\d{4}-\d{2}-\d{2}/.test(val)) return { $date: val };
    return val;
  }

  function buildSortQuery(): string | undefined {
    if (!sortField) return undefined;
    return JSON.stringify({ [sortField]: sortDir === 'asc' ? 1 : -1 });
  }

  async function executeQuery(p?: number) {
    const thisQuery = ++queryId;
    loading = true; selectedDoc = null; selectedIdx = -1;
    try {
      const pageNum = p !== undefined ? p : page;
      await documentStore.loadDocuments(connectionId, database, collection, pageNum, buildFilterQuery(), buildSortQuery());
      if (thisQuery !== queryId) return;
      documents = [...documentStore.documents];
      totalCount = documentStore.totalCount;
      page = documentStore.page;
      if (documents.length > 0) {
        const allKeys = new Set<string>(); allKeys.add('_id');
        documents.forEach(doc => Object.keys(doc).forEach(k => allKeys.add(k)));
        columns = Array.from(allKeys);
        for (const col of columns) {
          if (!fieldTypes[col]) getFieldType(col);
        }
      }
    } catch (e) { console.error('Query failed:', e); }
    if (thisQuery === queryId) loading = false;
  }

  $effect(() => {
    const key = `${connectionId}-${database}-${collection}`;
    if (key !== lastLoadKey && connectionId && database && collection) {
      lastLoadKey = key;
      page = 0; filters = {}; sortField = ''; sortDir = 'asc';
      executeQuery(0);
    }
    if (!connectionId || !database || !collection) {
      documents = []; columns = []; totalCount = 0; lastLoadKey = '';
    }
  });

  function handleSort(field: string) {
    if (sortField === field) { if (sortDir === 'asc') sortDir = 'desc'; else { sortField = ''; sortDir = 'asc'; } }
    else { sortField = field; sortDir = 'asc'; }
    executeQuery(0);
  }

  function openFilter(col: string) {
    filterDialogCol = col;
    filterDialogOp = filters[col]?.op || 'eq';
    filterDialogValue = filters[col]?.value || '';
    filterDialogOpen = true;
  }

  function applyFilter(op: string, value: string) {
    if (!value && !['isTrue','isFalse','exists','notExists','today','thisWeek','thisMonth','empty','notEmpty','isEmpty','notHasKey'].includes(op)) {
      const n = { ...filters }; delete n[filterDialogCol]; filters = n;
    } else {
      filters = { ...filters, [filterDialogCol]: { op, value } };
    }
    executeQuery(0);
  }

  function clearFilter(col: string) {
    const n = { ...filters }; delete n[col]; filters = n;
    filterDialogOpen = false;
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
  {:else if viewMode === 'table'}
    <div class="flex-1 overflow-hidden">
      <DataTable data={documents} {columns} {sortField} {sortDir} {filters}
        onRowClick={(doc, idx) => selectDoc(doc, idx)}
        onSort={(field, dir) => { sortField = field; sortDir = dir || 'asc'; executeQuery(0); }}
        onFilter={(col) => openFilter(col)} />
      {#if documents.length === 0}
        <div class="flex items-center justify-center py-8"><span class="text-[12px] text-[#7E97A7]">{Object.keys(filters).length > 0 ? 'No results match your filters' : 'No documents in this collection'}</span></div>
      {/if}
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

<FilterDialog bind:open={filterDialogOpen} column={filterDialogCol} {documents} currentOp={filterDialogOp} currentValue={filterDialogValue} hasFilter={!!filters[filterDialogCol]} onApply={applyFilter} onClear={() => clearFilter(filterDialogCol)} />

<ConfirmDialog bind:open={showDeleteConfirm} title="Delete Document" message="Are you sure you want to delete this document?" confirmText="Delete" variant="danger" onConfirm={handleDelete} />

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
