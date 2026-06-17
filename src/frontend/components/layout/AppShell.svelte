<script lang="ts">
  import { onMount } from 'svelte';
  import TitleBar from './TitleBar.svelte';
  import ActivityBar from './ActivityBar.svelte';
  import Sidebar from './Sidebar.svelte';
  import TabBar from './TabBar.svelte';
  import StatusBar from './StatusBar.svelte';
  import ToastContainer from '../common/ToastContainer.svelte';
  import ErrorBoundary from '../common/ErrorBoundary.svelte';
  
  let activeView = $state('connections');
  let connections = $state<any[]>([]);
  let activeConnectionId = $state<string | null>(null);
  let tabs = $state<{ id: string; title: string }[]>([]);
  let activeTabId = $state<string | null>(null);
  
  function handleAddConnection(conn: any) {
    connections = [...connections, { ...conn, id: crypto.randomUUID() }];
  }
  
  function handleSelectConnection(id: string) {
    activeConnectionId = id;
  }
  
  function handleAddTab() {
    const newTab = { id: crypto.randomUUID(), title: 'New Tab' };
    tabs = [...tabs, newTab];
    activeTabId = newTab.id;
  }
  
  function handleCloseTab(id: string) {
    tabs = tabs.filter(t => t.id !== id);
    if (activeTabId === id) {
      activeTabId = tabs.length > 0 ? tabs[0].id : null;
    }
  }
  
  function handleSelectTab(id: string) {
    activeTabId = id;
  }
</script>

<div class="flex h-screen flex-col bg-slate-50 dark:bg-slate-950">
  <TitleBar />
  
  <div class="flex flex-1 overflow-hidden">
    <ActivityBar {activeView} onViewChange={(v) => activeView = v} />
    <Sidebar
      {connections}
      {activeConnectionId}
      onSelectConnection={handleSelectConnection}
      onAddConnection={handleAddConnection}
    />
    
    <div class="flex flex-1 flex-col overflow-hidden">
      <TabBar
        {tabs}
        {activeTabId}
        onSelect={handleSelectTab}
        onClose={handleCloseTab}
        onAdd={handleAddTab}
      />
      
      <div class="flex-1 overflow-auto p-4">
        <ErrorBoundary>
          {#if activeView === 'connections'}
            <div class="flex h-full items-center justify-center">
              <div class="text-center">
                <h2 class="text-xl font-semibold text-slate-900 dark:text-slate-100">MongoDesk</h2>
                <p class="mt-2 text-sm text-slate-500 dark:text-slate-400">Select a connection to get started</p>
              </div>
            </div>
          {:else}
            <div class="flex h-full items-center justify-center">
              <p class="text-sm text-slate-400">Coming soon...</p>
            </div>
          {/if}
        </ErrorBoundary>
      </div>
    </div>
  </div>
  
  <StatusBar
    connectionStatus={activeConnectionId ? 'connected' : 'disconnected'}
    connectionName={connections.find(c => c.id === activeConnectionId)?.name || ''}
  />
</div>

<ToastContainer />
