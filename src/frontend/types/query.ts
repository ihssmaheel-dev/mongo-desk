export interface QueryParams {
  connection_id: string;
  database: string;
  collection: string;
  query_text: string;
  timeout_ms?: number;
}

export interface QueryResult {
  documents: Record<string, unknown>[];
  total_count: number;
  execution_time_ms: number;
}

export interface QueryValidation {
  is_valid: boolean;
  error_message?: string;
}

export interface QueryHistoryEntry {
  id: string;
  connection_id: string;
  database: string;
  collection: string;
  query_text: string;
  execution_time_ms?: number;
  result_count?: number;
  is_favorite: boolean;
  created_at: string;
}
