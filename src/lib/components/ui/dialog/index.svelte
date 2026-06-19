<script lang="ts">
  import { cn } from '$lib/utils';

  let {
    open = $bindable(false),
    class: className = '',
    children
  }: {
    open: boolean;
    class?: string;
    children?: any;
  } = $props();

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') open = false;
  }
</script>

{#if open}
  <div class="fixed inset-0 z-50 flex items-center justify-center" role="dialog" onkeydown={handleKeydown}>
    <button class="fixed inset-0 bg-black/80" aria-label="Close" onclick={() => open = false}></button>
    <div class={cn(
      'relative z-50 grid w-full max-w-lg gap-4 border bg-background p-6 shadow-lg sm:rounded-lg',
      className
    )}>
      {#if children}{@render children()}{/if}
    </div>
  </div>
{/if}
