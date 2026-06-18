<script lang="ts">
  import { onMount, onDestroy } from 'svelte';

  let { split = 'horizontal', ratio = 0.5, minSize = 100, onResize }: {
    split?: 'horizontal' | 'vertical';
    ratio?: number;
    minSize?: number;
    onResize?: (first: number, second: number) => void;
  } = $props();

  let containerSize = $state(0);
  let splitRatio = $state(ratio);
  let isDragging = $state(false);
  let container: HTMLDivElement;
  let isHorizontal = $derived(split === 'horizontal');

  const firstSize = $derived(Math.max(minSize, containerSize * splitRatio));
  const secondSize = $derived(containerSize - firstSize - 4);

  onMount(() => {
    const observer = new ResizeObserver((entries) => {
      containerSize = isHorizontal
        ? entries[0].contentRect.width
        : entries[0].contentRect.height;
    });
    observer.observe(container);
    return () => observer.disconnect();
  });

  function handleMouseDown(e: MouseEvent) {
    isDragging = true;
    e.preventDefault();
  }

  function handleMouseMove(e: MouseEvent) {
    if (!isDragging || !container) return;

    const rect = container.getBoundingClientRect();
    const pos = isHorizontal ? e.clientX - rect.left : e.clientY - rect.top;
    const newRatio = Math.max(0.1, Math.min(0.9, pos / containerSize));
    splitRatio = newRatio;
    onResize?.(containerSize * newRatio, containerSize * (1 - newRatio));
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
  class="flex {isHorizontal ? 'flex-row' : 'flex-col'} overflow-hidden"
  style="width: 100%; height: 100%"
>
  <div class="overflow-hidden" style="{isHorizontal ? 'width' : 'height'}: {firstSize}px">
    <slot name="first" />
  </div>

  <div
    class="flex-shrink-0 select-none {isHorizontal ? 'w-1 h-full cursor-col-resize' : 'h-1 w-full cursor-row-resize'} bg-[var(--border-subtle)] hover:bg-emerald-500/50 transition-colors"
    role="separator"
    tabindex="0"
    onmousedown={handleMouseDown}
    onkeydown={(e) => {
      const step = 0.05;
      if (e.key === 'ArrowRight' || e.key === 'ArrowDown') splitRatio = Math.min(0.9, splitRatio + step);
      if (e.key === 'ArrowLeft' || e.key === 'ArrowUp') splitRatio = Math.max(0.1, splitRatio - step);
    }}
  ></div>

  <div class="flex-1 overflow-hidden">
    <slot name="second" />
  </div>
</div>
