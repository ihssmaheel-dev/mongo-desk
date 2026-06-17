<script lang="ts">
  import type { ConnectionGroup } from '../../types/connection';
  
  let { group }: { group: ConnectionGroup } = $props();
</script>

<div class="flex items-center gap-2 px-3 py-1">
  <svg class="h-4 w-4 text-slate-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
  </svg>
  <span class="text-xs font-medium text-slate-600 dark:text-slate-400">{group.name}</span>
</div>
