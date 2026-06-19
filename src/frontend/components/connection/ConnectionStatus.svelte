<script lang="ts">
  let { status = 'disconnected' }: { status?: 'connected' | 'connecting' | 'error' | 'disconnected' } = $props();

  const statusColors = {
    connected: 'bg-[#00ED64] shadow-[0_0_6px_rgba(0,237,100,0.4)]',
    connecting: 'bg-[#FFC010]',
    error: 'bg-[#FF5C5C]',
    disconnected: 'bg-[#7E97A7]',
  };

  const statusLabels = {
    connected: 'Connected',
    connecting: 'Connecting...',
    error: 'Error',
    disconnected: 'Disconnected',
  };
</script>

<div class="flex items-center gap-1.5">
  <span class="h-2 w-2 rounded-full {statusColors[status]}"></span>
  <span class="text-[11px] text-[#7E97A7]">{statusLabels[status]}</span>
</div>
