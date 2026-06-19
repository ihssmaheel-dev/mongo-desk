<script lang="ts">
  import type { NewConnection } from '../../types/connection';

  let { onSave, onCancel }: { onSave: (conn: NewConnection) => void; onCancel: () => void } = $props();

  let name = $state('');
  let connectionString = $state('mongodb://localhost:27017');
  let connType = $state<'standalone' | 'replica_set' | 'sharded'>('standalone');
  let color = $state('#00ED64');
  let readOnly = $state(false);
  let sslEnabled = $state(false);

  function handleSubmit() {
    onSave({
      name,
      connection_string: connectionString,
      type: connType,
      color,
      read_only: readOnly,
      ssl_enabled: sslEnabled,
      ssl_config: null,
      group_id: null,
      tags: null,
    });
  }
</script>

<form onsubmit={handleSubmit} class="space-y-4">
  <div>
    <label for="conn-name" class="mb-1 block text-[11px] font-medium text-[#7E97A7]">Name</label>
    <input id="conn-name" type="text" bind:value={name} placeholder="My MongoDB Server" required
      class="flex h-10 w-full rounded-md border border-[#2D3A45] bg-[#0E1318] px-3 py-2 text-sm text-[#C3D4DE] placeholder-[#465A6B] outline-none focus:ring-2 focus:ring-[#00ED64] focus:ring-offset-0" />
  </div>

  <div>
    <label for="conn-type" class="mb-1 block text-[11px] font-medium text-[#7E97A7]">Connection Type</label>
    <select id="conn-type" bind:value={connType}
      class="flex h-10 w-full items-center rounded-md border border-[#2D3A45] bg-[#0E1318] px-3 py-2 text-sm text-[#C3D4DE] outline-none focus:ring-2 focus:ring-[#00ED64] focus:ring-offset-0">
      <option value="standalone">Standalone</option>
      <option value="replica_set">Replica Set</option>
      <option value="sharded">Sharded Cluster</option>
    </select>
  </div>

  <div>
    <label for="conn-string" class="mb-1 block text-[11px] font-medium text-[#7E97A7]">Connection String</label>
    <input id="conn-string" type="text" bind:value={connectionString} placeholder="mongodb://localhost:27017" required
      class="flex h-10 w-full rounded-md border border-[#2D3A45] bg-[#0E1318] px-3 py-2 font-mono text-sm text-[#C3D4DE] placeholder-[#465A6B] outline-none focus:ring-2 focus:ring-[#00ED64] focus:ring-offset-0" />
  </div>

  <div class="flex gap-6">
    <label class="flex items-center gap-2 cursor-pointer">
      <input type="checkbox" bind:checked={readOnly} class="h-4 w-4 rounded border-[#2D3A45] bg-[#0E1318]" />
      <span class="text-[12px] text-[#7E97A7]">Read Only</span>
    </label>
    <label class="flex items-center gap-2 cursor-pointer">
      <input type="checkbox" bind:checked={sslEnabled} class="h-4 w-4 rounded border-[#2D3A45] bg-[#0E1318]" />
      <span class="text-[12px] text-[#7E97A7]">SSL/TLS</span>
    </label>
  </div>

  <div>
    <label class="mb-1 block text-[11px] font-medium text-[#7E97A7]">Color</label>
    <div class="flex gap-2">
      {#each ['#00ED64', '#5DD0FF', '#B79CFF', '#FF8966', '#FF5C5C'] as c}
        <button type="button"
          class="h-6 w-6 rounded-full transition-all {color === c ? 'ring-2 ring-offset-2 ring-offset-[#1F2933] ring-[#C3D4DE] scale-110' : 'hover:scale-110'}"
          style="background-color: {c}"
          onclick={() => color = c}></button>
      {/each}
    </div>
  </div>

  <div class="flex justify-end gap-2 pt-2">
    <button type="button" class="rounded-md border border-[#2D3A45] px-4 py-2 text-[12px] text-[#7E97A7] hover:bg-[#2D3A45] hover:text-[#C3D4DE] transition-colors" onclick={onCancel}>Cancel</button>
    <button type="submit" class="rounded-md bg-[#00684A] px-4 py-2 text-[12px] font-medium text-white hover:bg-[#00C75A] transition-colors">Save</button>
  </div>
</form>
