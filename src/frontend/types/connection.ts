export interface Connection {
  id: string;
  name: string;
  connection_string: string;
  type: 'standalone' | 'replica_set' | 'sharded';
  color: string | null;
  read_only: boolean;
  ssl_enabled: boolean;
  ssl_config: string | null;
  group_id: string | null;
  tags: string | null;
  created_at: string;
  updated_at: string;
}

export interface NewConnection {
  name: string;
  connection_string: string;
  type: 'standalone' | 'replica_set' | 'sharded';
  color: string | null;
  read_only: boolean;
  ssl_enabled: boolean;
  ssl_config: string | null;
  group_id: string | null;
  tags: string | null;
}

export interface UpdateConnection {
  name: string | null;
  connection_string: string | null;
  type: 'standalone' | 'replica_set' | 'sharded' | null;
  color: string | null;
  read_only: boolean | null;
  ssl_enabled: boolean | null;
  ssl_config: string | null;
  group_id: string | null;
  tags: string | null;
}

export interface ConnectionGroup {
  id: string;
  name: string;
  parent_id: string | null;
  sort_order: number;
  created_at: string;
}

export interface NewConnectionGroup {
  name: string;
  parent_id: string | null;
}

export interface ConnectionTestResult {
  success: boolean;
  message: string;
  databases: string[] | null;
}

export interface ConnectionFavorite {
  id: string;
  connection_id: string;
  sort_order: number;
  created_at: string;
}
