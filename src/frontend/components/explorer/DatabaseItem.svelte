<script lang="ts">
  import type { DatabaseInfo } from '../../types/document';

  let { database, connectionId, isActive = false, onSelect, onToggle }: {
    database: DatabaseInfo;
    connectionId: string;
    isActive?: boolean;
    onSelect: (db: string) => void;
    onToggle: (db: string) => void;
  } = $props();

  let expanded = $state(false);

  function handleClick() {
    expanded = !expanded;
    onToggle(database.name);
    onSelect(database.name);
  }
</script>

<div>
  <button
    class="flex w-full items-center gap-2 rounded px-2 py-1 text-left text-[12px] transition-colors hover:bg-[var(--bg-surface)] {isActive ? 'bg-[var(--accent-primary-subtle)]' : ''}"
    onclick={handleClick}
  >
    <svg class="h-3 w-3 text-[var(--text-tertiary)] transition-transform duration-150 {expanded ? 'rotate-90' : ''}" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
      <path stroke-linecap="round" d="M9 5l7 7-7 7" />
    </svg>
    <svg class="h-3.5 w-3.5 {isActive ? 'text-[var(--accent-primary)]' : 'text-[var(--text-tertiary)]'}" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
      <path stroke-linecap="round" stroke-linejoin="round" d="M4 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H6a2 2 0 00-2 2z" />
    </svg>
    <span class="flex-1 truncate {isActive ? 'font-medium text-[var(--text-primary)]' : 'text-[var(--text-secondary)]'}">{database.name}</span>
  </button>
</div>
