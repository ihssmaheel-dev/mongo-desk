<script lang="ts">
  let {
    connectionStatus = 'disconnected',
    connectionName = '',
    database = '',
    collection = '',
  }: {
    connectionStatus?: 'connected' | 'connecting' | 'error' | 'disconnected';
    connectionName?: string;
    database?: string;
    collection?: string;
  } = $props();
</script>

<footer class="flex h-5 items-center justify-between border-t border-[#2D3A45] bg-[#161D24] px-3 select-none">
  <div class="flex items-center gap-3">
    <div class="flex items-center gap-1.5">
      <span class="h-1.5 w-1.5 rounded-full {connectionStatus === 'connected' ? 'bg-[#00ED64] shadow-[0_0_4px_#00ED64]' : connectionStatus === 'error' ? 'bg-[#FF5C5C]' : connectionStatus === 'connecting' ? 'bg-[#FFC010]' : 'bg-[#7E97A7]'}"></span>
      <span class="text-[10px] text-[#7E97A7]">{connectionStatus === 'connected' ? 'Connected' : connectionStatus === 'connecting' ? 'Connecting...' : connectionStatus === 'error' ? 'Error' : 'Disconnected'}</span>
    </div>
    {#if connectionName}
      <span class="text-[10px] text-[#C3D4DE]">{connectionName}</span>
    {/if}
    {#if database}
      <svg class="h-2.5 w-2.5 text-[#2D3A45]" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" d="M9 5l7 7-7 7"/></svg>
      <span class="text-[10px] text-[#C3D4DE]">{database}</span>
    {/if}
    {#if collection}
      <svg class="h-2.5 w-2.5 text-[#2D3A45]" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" d="M9 5l7 7-7 7"/></svg>
      <span class="text-[10px] text-[#C3D4DE]">{collection}</span>
    {/if}
  </div>
  <div class="flex items-center gap-3">
    <span class="text-[10px] text-[#465A6B]">MongoDesk v1.0</span>
  </div>
</footer>
