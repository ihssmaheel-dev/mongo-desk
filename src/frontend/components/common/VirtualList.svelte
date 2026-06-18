<script lang="ts">
  import { onMount } from 'svelte';

  let { items, itemHeight = 32, overscan = 5, children }: {
    items: any[];
    itemHeight?: number;
    overscan?: number;
    children: any;
  } = $props();

  let container: HTMLDivElement;
  let scrollTop = $state(0);
  let containerHeight = $state(0);

  onMount(() => {
    const observer = new ResizeObserver((entries) => {
      containerHeight = entries[0].contentRect.height;
    });
    observer.observe(container);
    return () => observer.disconnect();
  });

  const totalHeight = $derived(items.length * itemHeight);
  const startIndex = $derived(Math.max(0, Math.floor(scrollTop / itemHeight) - overscan));
  const endIndex = $derived(Math.min(items.length, Math.ceil((scrollTop + containerHeight) / itemHeight) + overscan));
  const visibleItems = $derived(items.slice(startIndex, endIndex));

  function handleScroll(e: Event) {
    scrollTop = (e.target as HTMLDivElement).scrollTop;
  }
</script>

<div
  bind:this={container}
  class="overflow-auto"
  onscroll={handleScroll}
>
  <div class="relative" style="height: {totalHeight}px">
    <div class="absolute left-0 right-0" style="top: {startIndex * itemHeight}px">
      {#each visibleItems as item, i (startIndex + i)}
        <div style="height: {itemHeight}px">
          {@render children(item, startIndex + i)}
        </div>
      {/each}
    </div>
  </div>
</div>
