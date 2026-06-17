<script lang="ts">
  let { title = 'MongoDesk' }: { title?: string } = $props();
  
  let isMaximized = $state(false);
</script>

<div
  class="flex h-9 items-center justify-between bg-brand-forest px-2"
  data-tauri-drag-region
>
  <div class="flex items-center gap-2">
    <span class="text-sm font-semibold text-white">🚀 {title}</span>
  </div>
  
  <div class="flex">
    <button
      class="flex h-9 w-12 items-center justify-center text-white/70 hover:bg-white/10 hover:text-white"
      onclick={() => {}}
    >
      <svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 12H4" />
      </svg>
    </button>
    <button
      class="flex h-9 w-12 items-center justify-center text-white/70 hover:bg-white/10 hover:text-white"
      onclick={() => {}}
    >
      <svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        {#if isMaximized}
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 9V4.5M9 4.5H4.5M9 4.5L4.5 9M15 15v4.5M15 19.5H4.5M15 19.5l4.5-4.5M4.5 4.5L9 9m0 0l4.5 4.5M9 9l4.5-4.5" />
        {:else}
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 8V4m0 0h4M4 4l5 5m11-1V4m0 0h-4m4 0l-5 5M4 16v4m0 0h4m-4 0l5-5m11 5l-5-5m5 5v-4m0 4h-4" />
        {/if}
      </svg>
    </button>
    <button
      class="flex h-9 w-12 items-center justify-center text-white/70 hover:bg-semantic-danger hover:text-white"
      onclick={() => {}}
    >
      <svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
      </svg>
    </button>
  </div>
</div>
