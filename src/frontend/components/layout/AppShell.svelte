<script lang="ts">
  import TitleBar from './TitleBar.svelte';
  import ActivityBar from './ActivityBar.svelte';
  import Sidebar from './Sidebar.svelte';
  import StatusBar from './StatusBar.svelte';
  import ToastContainer from '../common/ToastContainer.svelte';
  import ErrorBoundary from '../common/ErrorBoundary.svelte';
  import { connectionStore } from '../../stores/connectionStore';

  let activeView = $state('connections');
  let activeDatabase = $state('');
  let activeCollection = $state('');

  function handleSelectCollection(db: string, coll: string) {
    activeDatabase = db;
    activeCollection = coll;
  }
</script>

<div class="flex h-screen flex-col bg-[var(--bg-app)]">
  <TitleBar />

  <div class="flex flex-1 overflow-hidden">
    <ActivityBar {activeView} onViewChange={(v) => activeView = v} />
    <Sidebar onSelectCollection={handleSelectCollection} />

    <div class="flex flex-1 flex-col overflow-hidden">
      <div class="flex-1 overflow-auto p-4">
        <ErrorBoundary>
          {#if activeCollection}
            <div class="flex h-full items-center justify-center">
              <div class="text-center">
                <h2 class="text-xl font-semibold text-slate-900 dark:text-slate-100">{activeCollection}</h2>
                <p class="mt-2 text-sm text-slate-500 dark:text-slate-400">{connectionStore.activeConnection?.name} / {activeDatabase} / {activeCollection}</p>
              </div>
            </div>
          {:else if connectionStore.activeConnection}
            <div class="flex h-full items-center justify-center">
              <div class="text-center">
                <h2 class="text-xl font-semibold text-slate-900 dark:text-slate-100">{connectionStore.activeConnection.name}</h2>
                <p class="mt-2 text-sm text-slate-500 dark:text-slate-400">Select a collection to view documents</p>
              </div>
            </div>
          {:else}
            <div class="flex h-full items-center justify-center">
              <div class="text-center">
                <h2 class="text-xl font-semibold text-slate-900 dark:text-slate-100">MongoDesk</h2>
                <p class="mt-2 text-sm text-slate-500 dark:text-slate-400">Select a connection to get started</p>
              </div>
            </div>
          {/if}
        </ErrorBoundary>
      </div>
    </div>
  </div>

  <StatusBar
    connectionStatus={connectionStore.activeConnectionId ? 'connected' : 'disconnected'}
    connectionName={connectionStore.activeConnection?.name || ''}
    database={activeDatabase}
    collection={activeCollection}
  />
</div>

<ToastContainer />
