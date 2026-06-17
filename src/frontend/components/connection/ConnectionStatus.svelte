<script lang="ts">
  let { status }: { status: 'connected' | 'connecting' | 'error' | 'disconnected' } = $props();
  
  const colors = {
    connected: 'bg-semantic-success',
    connecting: 'bg-semantic-warning',
    error: 'bg-semantic-danger',
    disconnected: 'bg-slate-400',
  };
  
  const labels = {
    connected: 'Connected',
    connecting: 'Connecting...',
    error: 'Error',
    disconnected: 'Disconnected',
  };
</script>

<div class="flex items-center gap-2">
  <div class="h-2 w-2 rounded-full {colors[status]}"></div>
  <span class="text-xs text-slate-600 dark:text-slate-400">{labels[status]}</span>
</div>
