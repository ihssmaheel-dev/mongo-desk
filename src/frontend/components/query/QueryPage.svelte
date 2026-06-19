<script lang="ts">
  import { queryStore } from '../../stores/queryStore';

  let { connectionId, database, collections }: {
    connectionId: string;
    database: string;
    collections: string[];
  } = $props();

  let selectedCollection = $state(collections[0] || '');
  let queryText = $state(`db.${selectedCollection || 'collection'}.find({})`);
  let results = $state<any[]>([]);
  let executionTime = $state(0);
  let totalCount = $state(0);
  let isRunning = $state(false);
  let error = $state('');
  let viewMode = $state<'results' | 'json'>('results');

  $effect(() => {
    if (selectedCollection) {
      queryText = `db.${selectedCollection}.find({})`;
    }
  });

  async function runQuery() {
    if (!queryText.trim() || isRunning) return;
    error = '';
    isRunning = true;
    results = [];
    executionTime = 0;
    totalCount = 0;

    const startTime = Date.now();
    const { data, error: queryError } = await (await import('../../services/tauriBridge')).tauriInvoke<{
      documents: any[];
      total_count: number;
      execution_time_ms: number;
    }>('execute_query', {
      params: {
        connection_id: connectionId,
        database,
        collection: selectedCollection,
        query_text: queryText,
      }
    });

    executionTime = Date.now() - startTime;
    isRunning = false;

    if (queryError) {
      error = queryError.data?.message || 'Query failed';
    } else if (data) {
      results = data.documents;
      totalCount = data.total_count;
      executionTime = data.execution_time_ms;
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
      e.preventDefault();
      runQuery();
    }
  }
</script>

<div class="flex h-full flex-col bg-[#0E1318]">
  <div class="flex items-center gap-3 border-b border-[#2D3A45] px-4 py-2 bg-[#161D24]">
    <span class="text-[13px] font-semibold text-[#C3D4DE]">Query Runner</span>

    <select bind:value={selectedCollection}
      class="rounded border border-[#2D3A45] bg-[#0E1318] px-2 py-1 text-[11px] text-[#C3D4DE] outline-none focus:border-[#00ED64]">
      {#each collections as coll}
        <option value={coll}>{coll}</option>
      {/each}
    </select>

    <div class="flex items-center gap-1">
      <button class="flex items-center gap-1 rounded bg-[#00684A] px-3 py-1 text-[11px] text-white hover:bg-[#00C75A] transition-colors disabled:opacity-50"
        onclick={runQuery} disabled={isRunning}>
        <svg class="h-3 w-3" fill="currentColor" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>
        {isRunning ? 'Running...' : 'Run'}
      </button>
    </div>
  </div>

  <div class="flex-1 overflow-hidden border-b border-[#2D3A45]">
    <textarea
      bind:value={queryText}
      onkeydown={handleKeydown}
      class="w-full h-full resize-none bg-[#0E1318] p-4 font-mono text-[12px] text-[#C3D4DE] outline-none placeholder-[#465A6B]"
      placeholder="Enter query JSON: filter, sort, limit, skip"
      spellcheck="false"
    ></textarea>
  </div>

  {#if error}
    <div class="border-b border-[#2D3A45] bg-[#FF5C5C]/10 px-4 py-2">
      <span class="text-[11px] text-[#FF5C5C]">{error}</span>
    </div>
  {/if}

  <div class="flex items-center justify-between border-b border-[#2D3A45] px-4 py-1.5 bg-[#161D24]">
    <div class="flex items-center gap-3">
      <span class="text-[11px] text-[#7E97A7]">{totalCount.toLocaleString()} results</span>
      <span class="text-[10px] text-[#465A6B]">{executionTime}ms</span>
    </div>
    <div class="flex rounded border border-[#2D3A45]">
      <button class="px-2 py-0.5 text-[10px] {viewMode === 'results' ? 'bg-[#023430] text-[#00ED64]' : 'text-[#7E97A7]'}" onclick={() => viewMode = 'results'}>Table</button>
      <button class="px-2 py-0.5 text-[10px] {viewMode === 'json' ? 'bg-[#023430] text-[#00ED64]' : 'text-[#7E97A7]'}" onclick={() => viewMode = 'json'}>JSON</button>
    </div>
  </div>

  <div class="flex-1 overflow-auto">
    {#if isRunning}
      <div class="flex items-center justify-center h-full">
        <div class="text-center">
          <div class="mb-3 mx-auto h-8 w-8 animate-spin rounded-full border-2 border-[#00ED64] border-t-transparent"></div>
          <span class="text-[12px] text-[#7E97A7]">Executing query...</span>
        </div>
      </div>
    {:else if results.length === 0 && !error}
      <div class="flex items-center justify-center h-full">
        <span class="text-[12px] text-[#465A6B]">Run a query to see results</span>
      </div>
    {:else if viewMode === 'results'}
      <table class="w-full border-collapse">
        <thead class="sticky top-0 z-10 bg-[#0E1318]">
          <tr>
            {#each results.length > 0 ? Object.keys(results[0]) : [] as col}
              <th class="border-b border-[#2D3A45] px-3 py-2 text-left text-[10px] font-semibold uppercase tracking-wider text-[#7E97A7] whitespace-nowrap">
                {col === '_id' ? 'id' : col}
              </th>
            {/each}
          </tr>
        </thead>
        <tbody>
          {#each results as doc}
            <tr class="border-b border-[#1F2933] hover:bg-[#161D24]">
              {#each Object.keys(doc) as col}
                {@const val = doc[col]}
                <td class="px-3 py-1.5 text-[11px] font-mono
                  {val === null ? 'text-[#7E97A7] italic' :
                   typeof val === 'object' && val?.$oid ? 'text-[#FFC010]' :
                   typeof val === 'number' ? 'text-[#5DD0FF]' :
                   typeof val === 'boolean' ? 'text-[#FF8966]' :
                   'text-[#00ED64]'}">
                  <span class="max-w-[200px] truncate block">{typeof val === 'object' ? (val?.$oid || val?.$date ? String(val.$oid || val.$date) : Array.isArray(val) ? `[${val.length}]` : '{...}') : val === null ? 'null' : String(val)}</span>
                </td>
              {/each}
            </tr>
          {/each}
        </tbody>
      </table>
    {:else}
      <pre class="p-4 text-[11px] font-mono text-[#C3D4DE] whitespace-pre-wrap">{JSON.stringify(results, null, 2)}</pre>
    {/if}
  </div>
</div>
