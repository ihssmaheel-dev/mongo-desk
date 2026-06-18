import type { DatabaseInfo, CollectionInfo } from '../types/document';
import { tauriInvoke } from '../services/tauriBridge';

class ExplorerStore {
  databases: DatabaseInfo[] = [];
  expandedDatabases: Set<string> = new Set();
  collections: Map<string, CollectionInfo[]> = new Map();
  loading = false;

  async loadDatabases(connectionId: string) {
    this.loading = true;
    const { data, error } = await tauriInvoke<DatabaseInfo[]>('list_databases', { connectionId });
    if (!error && data) {
      this.databases = data;
    }
    this.loading = false;
  }

  async loadCollections(connectionId: string, database: string) {
    const { data, error } = await tauriInvoke<CollectionInfo[]>('list_collections', { connectionId, database });
    if (!error && data) {
      this.collections.set(database, data);
    }
  }

  toggleDatabase(connectionId: string, database: string) {
    if (this.expandedDatabases.has(database)) {
      this.expandedDatabases.delete(database);
    } else {
      this.expandedDatabases.add(database);
      if (!this.collections.has(database)) {
        this.loadCollections(connectionId, database);
      }
    }
  }

  isDatabaseExpanded(database: string): boolean {
    return this.expandedDatabases.has(database);
  }

  getCollections(database: string): CollectionInfo[] {
    return this.collections.get(database) || [];
  }

  reset() {
    this.databases = [];
    this.expandedDatabases.clear();
    this.collections.clear();
  }
}

export const explorerStore = new ExplorerStore();
