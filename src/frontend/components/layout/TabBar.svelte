<script lang="ts">
  let { tabs = [], activeTabId = null, onSelect, onClose, onAdd }: {
    tabs: { id: string; title: string; icon?: string }[];
    activeTabId?: string | null;
    onSelect: (id: string) => void;
    onClose: (id: string) => void;
    onAdd: () => void;
  } = $props();
</script>

<div class="flex h-9 items-center border-b border-slate-200 bg-slate-100 dark:border-slate-700 dark:bg-slate-900">
  <div class="flex flex-1 overflow-x-auto">
    {#each tabs as tab (tab.id)}
      <button
        class="flex h-9 items-center gap-2 border-r border-slate-200 px-3 text-xs transition-colors hover:bg-slate-200 dark:border-slate-700 dark:hover:bg-slate-800 {activeTabId === tab.id ? 'border-b-2 border-b-brand-evergreen bg-white text-slate-900 dark:bg-slate-800 dark:text-slate-100' : 'text-slate-500 dark:text-slate-400'}"
        onclick={() => onSelect(tab.id)}
      >
        <span class="truncate">{tab.title}</span>
        <button
          class="ml-1 rounded p-0.5 text-slate-400 hover:bg-slate-300 hover:text-slate-600 dark:hover:bg-slate-700 dark:hover:text-slate-300"
          onclick|stopPropagation={() => onClose(tab.id)}
        >
          <svg class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </button>
    {/each}
  </div>
  
  <button
    class="flex h-9 w-9 items-center justify-center text-slate-400 hover:bg-slate-200 hover:text-slate-600 dark:hover:bg-slate-800 dark:hover:text-slate-300"
    onclick={onAdd}
  >
    <svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
    </svg>
  </button>
</div>
