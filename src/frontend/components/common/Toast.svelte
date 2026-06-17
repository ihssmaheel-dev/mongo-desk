<script lang="ts">
  import type { ErrorEntry } from '../../stores/errorStore.svelte';
  
  let { error, onDismiss }: { error: ErrorEntry; onDismiss?: (id: string) => void } = $props();
  
  let bgColor = $derived({
    transient: 'bg-slate-100 dark:bg-slate-800',
    persistent: 'bg-semantic-danger/10 dark:bg-semantic-danger/20',
    fatal: 'bg-semantic-danger dark:bg-semantic-danger/30',
  }[error.severity]);
  
  let textColor = $derived({
    transient: 'text-slate-900 dark:text-slate-100',
    persistent: 'text-semantic-danger',
    fatal: 'text-white',
  }[error.severity]);
</script>

<div class="rounded-md {bgColor} p-4 shadow-lg">
  <div class="flex items-start">
    <div class="flex-1">
      <p class="text-sm font-medium {textColor}">
        {error.message}
      </p>
      {#if error.detail}
        <p class="mt-1 text-xs {textColor} opacity-75">
          {error.detail}
        </p>
      {/if}
    </div>
    <button
      aria-label="Dismiss"
      class="ml-4 {textColor} opacity-50 hover:opacity-100"
      onclick={() => onDismiss?.(error.id)}
    >
      <svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
      </svg>
    </button>
  </div>
</div>
