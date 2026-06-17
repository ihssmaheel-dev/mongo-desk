<script lang="ts">
  import type { NewConnection } from '../../types/connection';
  import ConnectionForm from './ConnectionForm.svelte';
  
  let { open = $bindable(false), onSave }: { open: boolean; onSave: (conn: NewConnection) => void } = $props();
  
  function handleSave(conn: NewConnection) {
    onSave(conn);
    open = false;
  }
  
  function handleCancel() {
    open = false;
  }
</script>

{#if open}
  <div class="fixed inset-0 z-50 flex items-center justify-center">
    <div class="fixed inset-0 bg-black/50" onclick={handleCancel}></div>
    <div class="relative w-full max-w-md rounded-md bg-white p-6 shadow-xl dark:bg-slate-800">
      <h2 class="text-lg font-semibold text-slate-900 dark:text-slate-100">New Connection</h2>
      <div class="mt-4">
        <ConnectionForm onSave={handleSave} onCancel={handleCancel} />
      </div>
    </div>
  </div>
{/if}
