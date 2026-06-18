<script lang="ts">
  import type { NewConnection } from '../../types/connection';

  let { open = $bindable(false), onSave }: { open: boolean; onSave: (conn: NewConnection) => void } = $props();

  let name = $state('');
  let connectionString = $state('mongodb://localhost:27017');
  let connType = $state<'standalone' | 'replica_set' | 'sharded'>('standalone');
  let color = $state('#00ED64');
  let readOnly = $state(false);
  let sslEnabled = $state(false);

  function handleSave() {
    if (!name.trim()) return;
    onSave({
      name: name.trim(),
      connection_string: connectionString,
      type: connType,
      color,
      read_only: readOnly,
      ssl_enabled: sslEnabled,
      ssl_config: null,
      group_id: null,
      tags: null,
    });
    name = '';
    connectionString = 'mongodb://localhost:27017';
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') open = false;
  }
</script>

{#if open}
  <div class="fixed inset-0 z-50 flex items-center justify-center" role="dialog" aria-modal="true" onkeydown={handleKeydown}>
    <button class="fixed inset-0 bg-black/60 backdrop-blur-sm" aria-label="Close" onclick={() => open = false}></button>
    <div class="relative w-full max-w-md rounded-lg bg-[#1F2933] shadow-2xl border border-[#2D3A45]">
      <div class="flex items-center justify-between border-b border-[#2D3A45] px-5 py-3">
        <h2 class="text-[14px] font-semibold text-[#C3D4DE]">New Connection</h2>
        <button class="rounded p-1 text-[#465A6B] hover:bg-[#2D3A45] hover:text-[#C3D4DE] transition-colors" onclick={() => open = false}>
          <svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" d="M6 18L18 6M6 6l12 12"/></svg>
        </button>
      </div>

      <div class="space-y-4 px-5 py-4">
        <div>
          <label for="conn-name" class="mb-1 block text-[11px] font-medium text-[#7E97A7]">Name</label>
          <input id="conn-name" type="text" bind:value={name} placeholder="My MongoDB Server"
            class="w-full rounded border border-[#2D3A45] bg-[#0E1318] px-3 py-2 text-[13px] text-[#C3D4DE] placeholder-[#465A6B] outline-none focus:border-[#00ED64] transition-colors" />
        </div>

        <div>
          <label for="conn-type" class="mb-1 block text-[11px] font-medium text-[#7E97A7]">Connection Type</label>
          <select id="conn-type" bind:value={connType}
            class="w-full rounded border border-[#2D3A45] bg-[#0E1318] px-3 py-2 text-[13px] text-[#C3D4DE] outline-none focus:border-[#00ED64] transition-colors">
            <option value="standalone">Standalone</option>
            <option value="replica_set">Replica Set</option>
            <option value="sharded">Sharded Cluster</option>
          </select>
        </div>

        <div>
          <label for="conn-string" class="mb-1 block text-[11px] font-medium text-[#7E97A7]">Connection String</label>
          <input id="conn-string" type="text" bind:value={connectionString} placeholder="mongodb://localhost:27017"
            class="w-full rounded border border-[#2D3A45] bg-[#0E1318] px-3 py-2 font-mono text-[12px] text-[#C3D4DE] placeholder-[#465A6B] outline-none focus:border-[#00ED64] transition-colors" />
        </div>

        <div class="flex gap-4">
          <label class="flex items-center gap-2 cursor-pointer">
            <input type="checkbox" bind:checked={readOnly} class="h-3.5 w-3.5 rounded border-[#2D3A45] bg-[#0E1318] accent-[#00ED64]" />
            <span class="text-[12px] text-[#7E97A7]">Read Only</span>
          </label>
          <label class="flex items-center gap-2 cursor-pointer">
            <input type="checkbox" bind:checked={sslEnabled} class="h-3.5 w-3.5 rounded border-[#2D3A45] bg-[#0E1318] accent-[#00ED64]" />
            <span class="text-[12px] text-[#7E97A7]">SSL/TLS</span>
          </label>
        </div>

        <div>
          <label class="mb-1 block text-[11px] font-medium text-[#7E97A7]">Color</label>
          <div class="flex gap-2">
            {#each ['#00ED64', '#5DD0FF', '#B79CFF', '#FF8966', '#FF5C5C'] as c}
              <button type="button"
                class="h-5 w-5 rounded-full transition-all {color === c ? 'ring-2 ring-offset-2 ring-offset-[#1F2933] ring-[#C3D4DE] scale-110' : 'hover:scale-110'}"
                style="background-color: {c}"
                onclick={() => color = c}></button>
            {/each}
          </div>
        </div>
      </div>

      <div class="flex justify-end gap-2 border-t border-[#2D3A45] px-5 py-3">
        <button class="rounded border border-[#2D3A45] px-4 py-1.5 text-[12px] text-[#7E97A7] hover:bg-[#2D3A45] hover:text-[#C3D4DE] transition-colors" onclick={() => open = false}>Cancel</button>
        <button class="rounded bg-[#00684A] px-4 py-1.5 text-[12px] font-medium text-white hover:bg-[#00C75A] transition-colors" onclick={handleSave}>Save</button>
      </div>
    </div>
  </div>
{/if}
