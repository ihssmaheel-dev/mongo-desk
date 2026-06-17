<script lang="ts">
  let { 
    connectionStatus = 'disconnected',
    connectionName = '',
    database = '',
    collection = '',
    documentCount = 0,
    queryTime = 0,
  }: {
    connectionStatus?: 'connected' | 'connecting' | 'error' | 'disconnected';
    connectionName?: string;
    database?: string;
    collection?: string;
    documentCount?: number;
    queryTime?: number;
  } = $props();
  
  const statusColors = {
    connected: 'bg-semantic-success',
    connecting: 'bg-semantic-warning',
    error: 'bg-semantic-danger',
    disconnected: 'bg-slate-400',
  };
</script>

<div class="flex h-6 items-center justify-between border-t border-slate-200 bg-slate-100 px-3 dark:border-slate-700 dark:bg-slate-900">
  <div class="flex items-center gap-4">
    <div class="flex items-center gap-1.5">
      <div class="h-2 w-2 rounded-full {statusColors[connectionStatus]}"></div>
      <span class="text-xs text-slate-600 dark:text-slate-400">{connectionName || 'Disconnected'}</span>
    </div>
    {#if database}
      <span class="text-xs text-slate-500 dark:text-slate-500">{database}</span>
    {/if}
    {#if collection}
      <span class="text-xs text-slate-500 dark:text-slate-500">{collection}</span>
    {/if}
    {#if documentCount > 0}
      <span class="text-xs text-slate-500 dark:text-slate-500">{documentCount.toLocaleString()} docs</span>
    {/if}
  </div>
  
  <div class="flex items-center gap-4">
    {#if queryTime > 0}
      <span class="text-xs text-slate-500 dark:text-slate-500">{queryTime}ms</span>
    {/if}
  </div>
</div>
