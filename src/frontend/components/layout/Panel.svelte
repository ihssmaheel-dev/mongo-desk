<script lang="ts">
  import { onMount, onDestroy } from 'svelte';

  let { minSize = 100, maxSize = 800, defaultSize = 260, position = 'left', onResize }: {
    minSize?: number;
    maxSize?: number;
    defaultSize?: number;
    position?: 'left' | 'right' | 'top' | 'bottom';
    onResize?: (size: number) => void;
  } = $props();

  let size = $state(defaultSize);
  let isDragging = $state(false);
  let container: HTMLDivElement;
  let isHorizontal = $derived(position === 'top' || position === 'bottom');

  function handleMouseDown(e: MouseEvent) {
    isDragging = true;
    e.preventDefault();
  }

  function handleMouseMove(e: MouseEvent) {
    if (!isDragging || !container) return;

    const rect = container.getBoundingClientRect();
    let newSize: number;

    if (isHorizontal) {
      newSize = position === 'top' ? e.clientY - rect.top : rect.bottom - e.clientY;
    } else {
      newSize = position === 'left' ? e.clientX - rect.left : rect.right - e.clientX;
    }

    size = Math.max(minSize, Math.min(maxSize, newSize));
    onResize?.(size);
  }

  function handleMouseUp() {
    isDragging = false;
  }

  onMount(() => {
    document.addEventListener('mousemove', handleMouseMove);
    document.addEventListener('mouseup', handleMouseUp);
  });

  onDestroy(() => {
    document.removeEventListener('mousemove', handleMouseMove);
    document.removeEventListener('mouseup', handleMouseUp);
  });
</script>

<div
  bind:this={container}
  class="flex {isHorizontal ? 'flex-col' : 'flex-row'} overflow-hidden"
  style={isHorizontal ? 'height: 100%' : 'width: 100%'}
>
  {#if position === 'right' || position === 'bottom'}
    <div class="flex-1 overflow-hidden">
      <slot />
    </div>
  {/if}

  <div
    class="flex-shrink-0 select-none {isHorizontal ? 'h-1 w-full cursor-row-resize' : 'w-1 h-full cursor-col-resize'} bg-[var(--border-subtle)] hover:bg-emerald-500/50 transition-colors"
    role="separator"
    tabindex="0"
    onmousedown={handleMouseDown}
    onkeydown={(e) => {
      if (e.key === 'ArrowRight' || e.key === 'ArrowDown') size = Math.min(maxSize, size + 10);
      if (e.key === 'ArrowLeft' || e.key === 'ArrowUp') size = Math.max(minSize, size - 10);
    }}
  ></div>

  {#if position === 'left' || position === 'top'}
    <div class="overflow-hidden" style={isHorizontal ? 'height' : 'width'}: {size}px>
      <slot />
    </div>
  {/if}
</div>
