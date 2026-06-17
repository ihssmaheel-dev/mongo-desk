<script lang="ts">
  import type { NewConnection } from '../../types/connection';
  
  let { onSave, onCancel }: { onSave: (conn: NewConnection) => void; onCancel: () => void } = $props();
  
  let name = $state('');
  let connectionString = $state('mongodb://localhost:27017');
  let connType = $state<'standalone' | 'replica_set' | 'sharded'>('standalone');
  let color = $state('#00ED64');
  let readOnly = $state(false);
  let sslEnabled = $state(false);
  let groupId = $state<string | null>(null);
  
  function handleSubmit() {
    onSave({
      name,
      connection_string: connectionString,
      type: connType,
      color,
      read_only: readOnly,
      ssl_enabled: sslEnabled,
      ssl_config: null,
      group_id: groupId,
      tags: null,
    });
  }
</script>

<form onsubmit={handleSubmit} class="space-y-4">
  <div>
    <label for="name" class="block text-xs font-medium text-slate-600 dark:text-slate-400">
      Name
    </label>
    <input
      id="name"
      type="text"
      bind:value={name}
      class="mt-1 block w-full rounded-sm border border-slate-200 bg-white px-3 py-2 text-sm text-slate-900 placeholder-slate-400 focus:border-brand-evergreen focus:outline-none focus:ring-1 focus:ring-brand-evergreen dark:border-slate-700 dark:bg-slate-800 dark:text-slate-100"
      placeholder="My MongoDB Server"
      required
    />
  </div>
  
  <div>
    <label for="type" class="block text-xs font-medium text-slate-600 dark:text-slate-400">
      Connection Type
    </label>
    <select
      id="type"
      bind:value={connType}
      class="mt-1 block w-full rounded-sm border border-slate-200 bg-white px-3 py-2 text-sm text-slate-900 focus:border-brand-evergreen focus:outline-none focus:ring-1 focus:ring-brand-evergreen dark:border-slate-700 dark:bg-slate-800 dark:text-slate-100"
    >
      <option value="standalone">Standalone</option>
      <option value="replica_set">Replica Set</option>
      <option value="sharded">Sharded Cluster</option>
    </select>
  </div>
  
  <div>
    <label for="connectionString" class="block text-xs font-medium text-slate-600 dark:text-slate-400">
      Connection String
    </label>
    <input
      id="connectionString"
      type="text"
      bind:value={connectionString}
      class="mt-1 block w-full rounded-sm border border-slate-200 bg-white px-3 py-2 font-mono text-sm text-slate-900 placeholder-slate-400 focus:border-brand-evergreen focus:outline-none focus:ring-1 focus:ring-brand-evergreen dark:border-slate-700 dark:bg-slate-800 dark:text-slate-100"
      placeholder="mongodb://localhost:27017"
      required
    />
  </div>
  
  <div class="flex items-center gap-2">
    <input
      id="readOnly"
      type="checkbox"
      bind:checked={readOnly}
      class="h-4 w-4 rounded border-slate-300 text-brand-evergreen focus:ring-brand-evergreen"
    />
    <label for="readOnly" class="text-sm text-slate-600 dark:text-slate-400">
      Read Only Mode
    </label>
  </div>
  
  <div class="flex items-center gap-2">
    <input
      id="sslEnabled"
      type="checkbox"
      bind:checked={sslEnabled}
      class="h-4 w-4 rounded border-slate-300 text-brand-evergreen focus:ring-brand-evergreen"
    />
    <label for="sslEnabled" class="text-sm text-slate-600 dark:text-slate-400">
      Use SSL/TLS
    </label>
  </div>
  
  <div>
    <label class="block text-xs font-medium text-slate-600 dark:text-slate-400">
      Color
    </label>
    <div class="mt-1 flex gap-2">
      {#each ['#00ED64', '#5DD0FF', '#B79CFF', '#FF8966', '#FF5C5C'] as c}
        <button
          type="button"
          class="h-6 w-6 rounded-full {color === c ? 'ring-2 ring-offset-2 ring-brand-evergreen' : ''}"
          style="background-color: {c}"
          onclick={() => color = c}
        />
      {/each}
    </div>
  </div>
  
  <div class="flex justify-end gap-2 pt-4">
    <button
      type="button"
      class="rounded-sm border border-slate-200 px-4 py-2 text-sm font-medium text-slate-600 hover:bg-slate-100 dark:border-slate-700 dark:text-slate-400 dark:hover:bg-slate-800"
      onclick={onCancel}
    >
      Cancel
    </button>
    <button
      type="submit"
      class="rounded-sm bg-brand-evergreen px-4 py-2 text-sm font-medium text-white hover:bg-brand-spring-dim"
    >
      Save
    </button>
  </div>
</form>
