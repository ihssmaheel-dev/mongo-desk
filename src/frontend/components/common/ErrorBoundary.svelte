<script lang="ts">
  let { children } = $props();
  let error = $state<any>(null);
</script>

{#if error}
  <div class="flex h-full flex-col items-center justify-center p-8">
    <svg class="mb-4 h-12 w-12 text-[#FF5C5C]" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
      <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
    </svg>
    <h2 class="mb-2 text-[16px] font-semibold text-[#C3D4DE]">Something went wrong</h2>
    <p class="mb-4 max-w-md text-center text-[13px] text-[#7E97A7]">{error?.message || 'An unexpected error occurred'}</p>
    <button class="rounded border border-[#2D3A45] px-4 py-2 text-[12px] text-[#7E97A7] hover:bg-[#2D3A45] hover:text-[#C3D4DE] transition-colors" onclick={() => { error = null; }}>Try Again</button>
  </div>
{:else}
  {@render children()}
{/if}
