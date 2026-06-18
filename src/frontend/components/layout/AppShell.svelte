<script lang="ts">
  import TitleBar from './TitleBar.svelte';
  import ActivityBar from './ActivityBar.svelte';
  import Sidebar from './Sidebar.svelte';
  import StatusBar from './StatusBar.svelte';
  import ToastContainer from '../common/ToastContainer.svelte';
  import ErrorBoundary from '../common/ErrorBoundary.svelte';
  import DocumentViewer from '../documents/DocumentViewer.svelte';
  import { connectionStore } from '../../stores/connectionStore';

  let activeView = $state('connections');
  let activeDatabase = $state('');
  let activeCollection = $state('');
  let activeConnectionId = $state('');

  function handleSelectCollection(db: string, coll: string) {
    activeDatabase = db;
    activeCollection = coll;
  }

  function handleSelectConnection(id: string) {
    activeConnectionId = id;
    activeDatabase = '';
    activeCollection = '';
  }
</script>

<div class="flex h-screen flex-col bg-[#0E1318]">
  <TitleBar />

  <div class="flex flex-1 overflow-hidden">
    <ActivityBar {activeView} onViewChange={(v) => activeView = v} />
    <Sidebar onSelectCollection={handleSelectCollection} onSelectConnection={handleSelectConnection} />

    <div class="flex flex-1 flex-col overflow-hidden">
      <div class="flex-1 overflow-hidden bg-[#0E1318]">
        <ErrorBoundary>
          {#if activeCollection && activeConnectionId}
            <DocumentViewer
              connectionId={activeConnectionId}
              database={activeDatabase}
              collection={activeCollection}
              onSelect={handleSelectCollection}
            />
          {:else if activeConnectionId}
            <div class="flex h-full items-center justify-center bg-[#0E1318]">
              <div class="text-center">
                <svg class="mx-auto mb-4 h-16 w-16 text-[#2D3A45]" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1"><path stroke-linecap="round" stroke-linejoin="round" d="M4 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H6a2 2 0 00-2 2z"/></svg>
                <h2 class="mb-1 text-[16px] font-semibold text-[#C3D4DE]">Connected</h2>
                <p class="text-[13px] text-[#7E97A7]">Select a collection from the sidebar to view documents</p>
              </div>
            </div>
          {:else}
            <div class="flex h-full items-center justify-center bg-[#0E1318]">
              <div class="text-center">
                <div class="mb-4 flex items-center justify-center gap-3">
                  <div class="flex h-12 w-12 items-center justify-center rounded-xl bg-gradient-to-br from-[#00684A] to-[#00ED64]">
                    <svg class="h-6 w-6 text-white" viewBox="0 0 24 24" fill="currentColor"><path d="M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10 10-4.5 10-10S17.5 2 12 2zm-1 15.93c-3.95-.49-7-3.85-7-7.93 0-.62.08-1.21.21-1.79L9 15v1c0 1.1.9 2 2 2v1.93zm6.9-2.54c-.26-.81-1-1.39-1.9-1.39h-1v-3c0-.55-.45-1-1-1H8v-2h2c.55 0 1-.45 1-1V7h2c1.1 0 2-.9 2-2v-.41c2.93 1.19 5 4.06 5 7.41 0 2.08-.8 3.97-2.1 5.39z"/></svg>
                  </div>
                  <h1 class="text-[24px] font-bold text-[#C3D4DE]">MongoDesk</h1>
                </div>
                <p class="text-[14px] text-[#7E97A7]">Select or create a connection to get started</p>
              </div>
            </div>
          {/if}
        </ErrorBoundary>
      </div>
    </div>
  </div>

  <StatusBar
    connectionStatus={activeConnectionId ? 'connected' : 'disconnected'}
    connectionName={connectionStore.connections.find(c => c.id === activeConnectionId)?.name || ''}
    database={activeDatabase}
    collection={activeCollection}
  />
</div>

<ToastContainer />
