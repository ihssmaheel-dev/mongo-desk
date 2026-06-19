import type { QueryResult, QueryHistoryEntry } from '../types/query';
import { tauriInvoke } from '../services/tauriBridge';

class QueryStore {
  queryText = '';
  results: any[] = [];
  executionTime = 0;
  totalCount = 0;
  isRunning = false;
  history: QueryHistoryEntry[] = [];

  async executeQuery(connectionId: string, database: string, collection: string, queryText: string) {
    this.isRunning = true;
    this.queryText = queryText;
    this.results = [];
    this.executionTime = 0;
    this.totalCount = 0;

    const startTime = Date.now();
    const { data, error } = await tauriInvoke<QueryResult>('execute_query', {
      params: {
        connection_id: connectionId,
        database,
        collection,
        query_text: queryText,
      }
    });

    this.executionTime = Date.now() - startTime;
    this.isRunning = false;

    if (!error && data) {
      this.results = data.documents;
      this.totalCount = data.total_count;
      this.executionTime = data.execution_time_ms;
    } else {
      this.results = [];
      this.totalCount = 0;
    }

    return !error;
  }

  async validateQuery(queryText: string) {
    const { data, error } = await tauriInvoke<{ is_valid: boolean; error_message?: string }>('validate_query', {
      query_text: queryText,
    });
    return error ? { is_valid: false, error_message: 'Validation failed' } : data;
  }

  reset() {
    this.queryText = '';
    this.results = [];
    this.executionTime = 0;
    this.totalCount = 0;
    this.isRunning = false;
  }
}

export const queryStore = new QueryStore();
