<script lang="ts">
  let { open = $bindable(false), title = 'Confirm', message = '', confirmText = 'Delete', cancelText = 'Cancel', variant = 'danger', onConfirm }: {
    open: boolean;
    title?: string;
    message?: string;
    confirmText?: string;
    cancelText?: string;
    variant?: 'danger' | 'warning' | 'info';
    onConfirm: () => void;
  } = $props();

  const variantStyles = {
    danger: { icon: 'M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16', color: '#FF5C5C', bg: '#FF5C5C' },
    warning: { icon: 'M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z', color: '#FFC010', bg: '#FFC010' },
    info: { icon: 'M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z', color: '#5DD0FF', bg: '#5DD0FF' },
  };

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') open = false;
  }

  function handleConfirm() {
    onConfirm();
    open = false;
  }
</script>

{#if open}
  <div class="fixed inset-0 z-50 flex items-center justify-center" role="dialog" aria-modal="true" onkeydown={handleKeydown}>
    <button class="fixed inset-0 bg-black/60 backdrop-blur-sm" aria-label="Close" onclick={() => open = false}></button>
    <div class="relative w-full max-w-sm rounded-lg bg-[#1F2933] shadow-2xl border border-[#2D3A45]">
      <div class="px-5 pt-5 pb-2">
        <div class="mb-3 flex items-center gap-3">
          <div class="flex h-8 w-8 items-center justify-center rounded-full" style="background-color: {variantStyles[variant].bg}20">
            <svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke={variantStyles[variant].color} stroke-width="2">
              <path stroke-linecap="round" stroke-linejoin="round" d={variantStyles[variant].icon} />
            </svg>
          </div>
          <h3 class="text-[14px] font-semibold text-[#C3D4DE]">{title}</h3>
        </div>
        <p class="text-[13px] text-[#7E97A7] leading-relaxed">{message}</p>
      </div>
      <div class="flex justify-end gap-2 border-t border-[#2D3A45] px-5 py-3">
        <button class="rounded border border-[#2D3A45] px-4 py-1.5 text-[12px] text-[#7E97A7] hover:bg-[#2D3A45] hover:text-[#C3D4DE] transition-colors" onclick={() => open = false}>{cancelText}</button>
        <button
          class="rounded px-4 py-1.5 text-[12px] font-medium text-white transition-colors"
          style="background-color: {variantStyles[variant].bg}"
          onclick={handleConfirm}
        >{confirmText}</button>
      </div>
    </div>
  </div>
{/if}
