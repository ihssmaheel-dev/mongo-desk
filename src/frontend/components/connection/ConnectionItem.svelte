<script lang="ts">
  import type { Connection } from '../../types/connection';

  let { connection, isActive = false, onClick, onDelete }: {
    connection: Connection;
    isActive?: boolean;
    onClick: () => void;
    onDelete: (id: string) => void;
  } = $props();
</script>

<button
  class="group flex w-full items-center gap-2 rounded px-2.5 py-1.5 text-left transition-colors hover:bg-[#1F2933] {isActive ? 'bg-[#023430] text-[#C3D4DE]' : 'text-[#C3D4DE]'}"
  onclick={onClick}
>
  <div class="h-2 w-2 rounded-full bg-[#00ED64] shadow-[0_0_6px_rgba(0,237,100,0.4)]"></div>
  <span class="flex-1 truncate text-[12px] {isActive ? 'font-medium' : ''}">{connection.name}</span>
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <span
    class="hidden group-hover:flex h-5 w-5 items-center justify-center rounded text-[#465A6B] hover:bg-[#FF5C5C]/10 hover:text-[#FF5C5C] transition-colors"
    onclick={(e) => { e.stopPropagation(); onDelete(connection.id); }}
  >
    <svg class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
      <path stroke-linecap="round" d="M6 18L18 6M6 6l12 12" />
    </svg>
  </span>
</button>
