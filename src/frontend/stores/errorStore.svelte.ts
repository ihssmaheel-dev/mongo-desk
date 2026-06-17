import type { AppError } from '../services/tauriBridge';

export type ErrorSeverity = 'transient' | 'persistent' | 'fatal';

export interface ErrorEntry {
  id: string;
  code: string;
  message: string;
  detail?: string;
  severity: ErrorSeverity;
  timestamp: number;
  dismissed: boolean;
}

function createErrorStore() {
  let errors = $state<ErrorEntry[]>([]);

  function add(error: AppError, severity: ErrorSeverity = 'transient') {
    const entry: ErrorEntry = {
      id: crypto.randomUUID(),
      code: error.data.code,
      message: error.data.message,
      detail: error.data.detail,
      severity,
      timestamp: Date.now(),
      dismissed: false,
    };
    errors = [...errors, entry];

    if (severity === 'transient') {
      setTimeout(() => dismiss(entry.id), 5000);
    }
  }

  function dismiss(id: string) {
    errors = errors.map(e => e.id === id ? { ...e, dismissed: true } : e);
  }

  function dismissAll() {
    errors = errors.map(e => ({ ...e, dismissed: true }));
  }

  const active = $derived(errors.filter(e => !e.dismissed));
  const transient = $derived(active.filter(e => e.severity === 'transient'));
  const persistent = $derived(active.filter(e => e.severity === 'persistent'));
  const fatal = $derived(active.find(e => e.severity === 'fatal'));

  return { add, dismiss, dismissAll, active, transient, persistent, fatal };
}

export const errorStore = createErrorStore();
