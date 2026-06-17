import { invoke } from '@tauri-apps/api/core';

export interface AppError {
  type: 'connection' | 'query' | 'auth' | 'validation' | 'io' | 'internal';
  data: {
    code: string;
    message: string;
    detail?: string;
    field?: string;
  };
}

export interface TauriResult<T> {
  data: T | null;
  error: AppError | null;
}

export async function tauriInvoke<T>(
  command: string,
  args?: Record<string, unknown>
): Promise<TauriResult<T>> {
  try {
    const data = await invoke<T>(command, args);
    return { data, error: null };
  } catch (err) {
    const error: AppError = typeof err === 'string'
      ? JSON.parse(err)
      : { type: 'internal', data: { code: 'ERR_UNKNOWN', message: String(err) } };
    return { data: null, error };
  }
}
