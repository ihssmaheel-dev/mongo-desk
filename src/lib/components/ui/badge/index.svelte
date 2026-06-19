<script lang="ts">
  import { cn } from '$lib/utils';

  let {
    class: className = '',
    children,
    ...restProps
  }: {
    class?: string;
    children?: any;
  } = $props();
</script>

<div
  class={cn(
    'inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2',
    className
  )}
  {...restProps}
>
  {#if children}{@render children()}{/if}
</div>
