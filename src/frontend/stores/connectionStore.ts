import type { Connection, NewConnection, UpdateConnection, ConnectionGroup, NewConnectionGroup, ConnectionTestResult } from '../types/connection';
import { tauriInvoke } from '../services/tauriBridge';

class ConnectionStore {
  connections: Connection[] = [];
  groups: ConnectionGroup[] = [];
  activeConnectionId: string | null = null;
  loading = false;

  get activeConnection(): Connection | null {
    return this.connections.find(c => c.id === this.activeConnectionId) ?? null;
  }

  async loadConnections() {
    this.loading = true;
    const { data, error } = await tauriInvoke<Connection[]>('list_connections');
    if (!error && data) {
      this.connections = data;
    }
    this.loading = false;
  }

  async loadGroups() {
    const { data, error } = await tauriInvoke<ConnectionGroup[]>('list_connection_groups');
    if (!error && data) {
      this.groups = data;
    }
  }

  async addConnection(connection: NewConnection): Promise<Connection | null> {
    const { data, error } = await tauriInvoke<Connection>('add_connection', { connection });
    if (!error && data) {
      this.connections = [...this.connections, data];
      return data;
    }
    return null;
  }

  async updateConnection(id: string, connection: UpdateConnection): Promise<Connection | null> {
    const { data, error } = await tauriInvoke<Connection>('update_connection', { id, connection });
    if (!error && data) {
      this.connections = this.connections.map(c => c.id === id ? data : c);
      return data;
    }
    return null;
  }

  async deleteConnection(id: string): Promise<boolean> {
    const { error } = await tauriInvoke<void>('delete_connection', { id });
    if (!error) {
      this.connections = this.connections.filter(c => c.id !== id);
      if (this.activeConnectionId === id) {
        this.activeConnectionId = null;
      }
      return true;
    }
    return false;
  }

  async testConnection(id: string): Promise<ConnectionTestResult | null> {
    const { data, error } = await tauriInvoke<ConnectionTestResult>('test_connection', { id });
    return error ? null : data;
  }

  setActiveConnection(id: string | null) {
    this.activeConnectionId = id;
  }

  async addGroup(group: NewConnectionGroup): Promise<ConnectionGroup | null> {
    const { data, error } = await tauriInvoke<ConnectionGroup>('add_connection_group', { group });
    if (!error && data) {
      this.groups = [...this.groups, data];
      return data;
    }
    return null;
  }

  async importConnections(json: string): Promise<Connection[]> {
    const { data, error } = await tauriInvoke<Connection[]>('import_connections', { data: json });
    if (!error && data) {
      this.connections = [...this.connections, ...data];
      return data;
    }
    return [];
  }

  async exportConnections(): Promise<string | null> {
    const { data, error } = await tauriInvoke<string>('export_connections');
    return error ? null : data;
  }
}

export const connectionStore = new ConnectionStore();
