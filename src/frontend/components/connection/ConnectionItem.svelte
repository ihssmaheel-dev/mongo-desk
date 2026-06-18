<script lang="ts">
  import type { Connection } from '../../types/connection';

  let { connection, isActive = false, onClick }: { connection: Connection; isActive?: boolean; onClick: () => void } = $props();
</script>

<button
  class="flex w-full items-center gap-2 rounded px-2.5 py-1.5 text-left transition-colors hover:bg-[#1F2933] {isActive ? 'bg-[#023430] text-[#C3D4DE]' : 'text-[#C3D4DE]'}"
  onclick={onClick}
>
  <div class="h-2 w-2 rounded-full bg-[#00ED64] shadow-[0_0_6px_rgba(0,237,100,0.4)]"></div>
  <span class="flex-1 truncate text-[12px] {isActive ? 'font-medium' : ''}">{connection.name}</span>
</button>
