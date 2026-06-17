import type { Connection, NewConnection, UpdateConnection, ConnectionGroup, NewConnectionGroup, ConnectionTestResult } from '../types/connection';
import { tauriInvoke } from '../services/tauriBridge';

export async function addConnection(connection: NewConnection): Promise<Connection | null> {
  const { data, error } = await tauriInvoke<Connection>('add_connection', { connection });
  return data;
}

export async function updateConnection(id: string, connection: UpdateConnection): Promise<Connection | null> {
  const { data, error } = await tauriInvoke<Connection>('update_connection', { id, connection });
  return data;
}

export async function deleteConnection(id: string): Promise<boolean> {
  const { data, error } = await tauriInvoke<void>('delete_connection', { id });
  return !error;
}

export async function testConnection(id: string): Promise<ConnectionTestResult | null> {
  const { data, error } = await tauriInvoke<ConnectionTestResult>('test_connection', { id });
  return data;
}

export async function listConnections(): Promise<Connection[]> {
  const { data, error } = await tauriInvoke<Connection[]>('list_connections');
  return data || [];
}

export async function listConnectionGroups(): Promise<ConnectionGroup[]> {
  const { data, error } = await tauriInvoke<ConnectionGroup[]>('list_connection_groups');
  return data || [];
}

export async function addConnectionGroup(group: NewConnectionGroup): Promise<ConnectionGroup | null> {
  const { data, error } = await tauriInvoke<ConnectionGroup>('add_connection_group', { group });
  return data;
}

export async function importConnections(data: string): Promise<Connection[]> {
  const { data: connections, error } = await tauriInvoke<Connection[]>('import_connections', { data });
  return connections || [];
}

export async function exportConnections(): Promise<string | null> {
  const { data, error } = await tauriInvoke<string>('export_connections');
  return data;
}
