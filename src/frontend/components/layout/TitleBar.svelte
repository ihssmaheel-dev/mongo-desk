<script lang="ts">
  import { getCurrentWindow } from '@tauri-apps/api/window';

  let { title = 'MongoDesk' }: { title?: string } = $props();

  const appWindow = getCurrentWindow();

  function minimize() {
    appWindow.minimize();
  }

  function toggleMaximize() {
    appWindow.toggleMaximize();
  }

  function close() {
    appWindow.close();
  }
</script>

<header class="flex h-8 items-center justify-between bg-[#023430] px-3 select-none" data-tauri-drag-region>
  <div class="flex items-center gap-2">
    <div class="flex h-5 w-5 items-center justify-center rounded bg-gradient-to-br from-[#00ED64] to-[#00684A]">
      <svg class="h-3 w-3 text-white" viewBox="0 0 24 24" fill="currentColor"><path d="M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10 10-4.5 10-10S17.5 2 12 2zm-1 15.93c-3.95-.49-7-3.85-7-7.93 0-.62.08-1.21.21-1.79L9 15v1c0 1.1.9 2 2 2v1.93zm6.9-2.54c-.26-.81-1-1.39-1.9-1.39h-1v-3c0-.55-.45-1-1-1H8v-2h2c.55 0 1-.45 1-1V7h2c1.1 0 2-.9 2-2v-.41c2.93 1.19 5 4.06 5 7.41 0 2.08-.8 3.97-2.1 5.39z"/></svg>
    </div>
    <span class="text-[12px] font-semibold text-white">{title}</span>
  </div>

  <div class="flex">
    <button
      data-tauri-drag-region="false"
      aria-label="Minimize"
      class="flex h-8 w-10 items-center justify-center text-white/60 hover:bg-white/10 hover:text-white transition-colors"
      onclick={minimize}
    >
      <svg class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
        <path stroke-linecap="round" d="M20 12H4" />
      </svg>
    </button>
    <button
      data-tauri-drag-region="false"
      aria-label="Maximize"
      class="flex h-8 w-10 items-center justify-center text-white/60 hover:bg-white/10 hover:text-white transition-colors"
      onclick={toggleMaximize}
    >
      <svg class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
        <path stroke-linecap="round" stroke-linejoin="round" d="M4 8V4m0 0h4M4 4l5 5m11-1V4m0 0h-4m4 0l-5 5M4 16v4m0 0h4m-4 0l5-5m11 5l-5-5m5 5v-4m0 4h-4" />
      </svg>
    </button>
    <button
      data-tauri-drag-region="false"
      aria-label="Close"
      class="flex h-8 w-10 items-center justify-center text-white/60 hover:bg-[#FF5C5C] hover:text-white transition-colors"
      onclick={close}
    >
      <svg class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
        <path stroke-linecap="round" d="M6 18L18 6M6 6l12 12" />
      </svg>
    </button>
  </div>
</header>
