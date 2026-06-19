<script lang="ts">
  let { entry, onDismiss }: { entry: any; onDismiss: (id: string) => void } = $props();
</script>

<div class="flex items-start gap-3 rounded-lg border border-[#2D3A45] border-l-4 bg-[#1F2933] p-3 shadow-lg {entry.severity === 'persistent' || entry.severity === 'fatal' ? 'border-l-[#FF5C5C] bg-[#FF5C5C]/10' : 'border-l-[#FFC010] bg-[#FFC010]/10'}">
  <span class="text-[14px] {entry.severity === 'persistent' || entry.severity === 'fatal' ? 'text-[#FF5C5C]' : 'text-[#FFC010]'}">
    {#if entry.severity === 'transient'}⚠{:else}✕{/if}
  </span>
  <div class="flex-1">
    <p class="text-[12px] text-[#C3D4DE]">{entry.message}</p>
    {#if entry.detail}
      <p class="mt-1 text-[10px] text-[#7E97A7]">{entry.detail}</p>
    {/if}
  </div>
  <button class="rounded p-0.5 text-[#465A6B] hover:text-[#C3D4DE] transition-colors" onclick={() => onDismiss(entry.id)}>
    <svg class="h-3.5 w-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
      <path stroke-linecap="round" d="M6 18L18 6M6 6l12 12" />
    </svg>
  </button>
</div>
