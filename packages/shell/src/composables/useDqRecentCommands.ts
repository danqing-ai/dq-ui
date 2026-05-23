import { ref } from 'vue';

export interface UseDqRecentCommandsOptions {
  storageKey?: string;
  maxItems?: number;
}

export function useDqRecentCommands(options: UseDqRecentCommandsOptions = {}) {
  const storageKey = options.storageKey ?? 'dq-shell:recent-commands';
  const maxItems = options.maxItems ?? 8;
  const recentIds = ref<string[]>([]);

  function load() {
    if (typeof localStorage === 'undefined') return;
    try {
      const raw = localStorage.getItem(storageKey);
      if (!raw) return;
      const parsed = JSON.parse(raw);
      if (!Array.isArray(parsed)) return;
      recentIds.value = parsed.filter((item): item is string => typeof item === 'string').slice(0, maxItems);
    } catch {
      recentIds.value = [];
    }
  }

  function persist() {
    if (typeof localStorage === 'undefined') return;
    try {
      localStorage.setItem(storageKey, JSON.stringify(recentIds.value));
    } catch {
      // Ignore quota/privacy mode failures.
    }
  }

  function markUsed(id: string) {
    const next = [id, ...recentIds.value.filter((item) => item !== id)].slice(0, maxItems);
    recentIds.value = next;
    persist();
  }

  load();

  return {
    recentIds,
    markUsed,
  };
}
