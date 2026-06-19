<script lang="ts">
  let { tabs, activeTabId, onSelect, onClose, onAdd }: {
    tabs: { id: string; title: string; type: string }[];
    activeTabId: string | null;
    onSelect: (id: string) => void;
    onClose: (id: string) => void;
    onAdd: () => void;
  } = $props();
</script>

<div class="flex items-center border-b border-[#2D3A45] bg-[#0E1318]">
  <div class="flex flex-1 overflow-x-auto">
    {#each tabs as tab (tab.id)}
      {@const isActive = tab.id === activeTabId}
      <button
        class="group relative flex items-center gap-2 border-r border-[#2D3A45] px-3 py-1.5 text-[11px] transition-colors {isActive ? 'bg-[#161D24] text-[#C3D4DE]' : 'text-[#465A6B] hover:bg-[#161D24] hover:text-[#7E97A7]'}"
        onclick={() => onSelect(tab.id)}
      >
        {#if isActive}
          <div class="absolute bottom-0 left-0 h-0.5 w-full bg-[#00ED64]"></div>
        {/if}
        <svg class="h-3 w-3 {isActive ? 'text-[#00ED64]' : 'text-[#465A6B]'}" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
          {#if tab.type === 'collection'}
            <path stroke-linecap="round" stroke-linejoin="round" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
          {:else}
            <path stroke-linecap="round" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4" />
          {/if}
        </svg>
        <span class="truncate max-w-[120px]">{tab.title}</span>
        <span
          role="button"
          tabindex="0"
          class="ml-1 rounded p-0.5 opacity-0 transition-opacity hover:bg-[#2D3A45] group-hover:opacity-100"
          onclick={(e) => { e.stopPropagation(); onClose(tab.id); }}
          onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') { e.stopPropagation(); onClose(tab.id); } }}
        >
          <svg class="h-2.5 w-2.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
            <path stroke-linecap="round" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </span>
      </button>
    {/each}
  </div>
  <button class="flex h-full items-center justify-center border-l border-[#2D3A45] px-3 text-[#465A6B] hover:bg-[#161D24] hover:text-[#C3D4DE] transition-colors" onclick={onAdd}>
    <svg class="h-3.5 w-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
      <path stroke-linecap="round" d="M12 4v16m8-8H4" />
    </svg>
  </button>
</div>
