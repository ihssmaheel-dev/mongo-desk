<script lang="ts">
  import type { ConnectionGroup } from '../../types/connection';

  let { group, isActive = false, onClick }: { group: ConnectionGroup; isActive?: boolean; onClick: () => void } = $props();
</script>

<button
  class="flex w-full items-center gap-2 rounded px-2 py-1.5 text-left transition-colors hover:bg-[#1F2933] {isActive ? 'bg-[#023430] text-[#C3D4DE]' : 'text-[#C3D4DE]'}"
  onclick={onClick}
>
  <svg class="h-3.5 w-3.5 text-[#7E97A7]" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
    <path stroke-linecap="round" stroke-linejoin="round" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
  </svg>
  <span class="flex-1 truncate text-[12px]">{group.name}</span>
</button>
