<script lang="ts">
  import { onMount } from 'svelte';
  
  let { children }: { children: any } = $props();
  let error = $state<any>(null);

  function handleError(event: ErrorEvent) {
    error = event.error;
  }

  onMount(() => {
    window.addEventListener('error', handleError);
    return () => window.removeEventListener('error', handleError);
  });
</script>

{#if error}
  <div class="flex h-full flex-col items-center justify-center p-8">
    <h2 class="text-lg font-semibold text-semantic-danger">Something went wrong</h2>
    <p class="mt-2 text-sm text-slate-400">{error.message}</p>
    <button
      class="mt-4 rounded-sm bg-brand-spring px-4 py-2 text-sm font-medium text-slate-950 hover:bg-brand-spring-dim"
      onclick={() => { error = null; }}
    >
      Try Again
    </button>
  </div>
{:else}
  {@render children()}
{/if}
