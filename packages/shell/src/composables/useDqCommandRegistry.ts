import { inject, provide, ref, type InjectionKey, type Ref } from 'vue';
import type { DqCommandAction } from '../components/commandPaletteTypes';

export interface DqCommandRegistry {
  actions: Ref<DqCommandAction[]>;
  registerCommand: (action: DqCommandAction) => () => void;
  registerCommands: (actions: DqCommandAction[]) => () => void;
  unregisterCommand: (id: string) => void;
  clearCommands: () => void;
}

const registryKey: InjectionKey<DqCommandRegistry> = Symbol('dq-shell-command-registry');

export function createDqCommandRegistry(): DqCommandRegistry {
  const map = new Map<string, DqCommandAction>();
  const actions = ref<DqCommandAction[]>([]);

  const refresh = () => {
    actions.value = [...map.values()];
  };

  const unregisterCommand = (id: string) => {
    map.delete(id);
    refresh();
  };

  const registerCommand = (action: DqCommandAction) => {
    map.set(action.id, action);
    refresh();
    return () => unregisterCommand(action.id);
  };

  const registerCommands = (items: DqCommandAction[]) => {
    for (const item of items) map.set(item.id, item);
    refresh();
    return () => {
      for (const item of items) map.delete(item.id);
      refresh();
    };
  };

  const clearCommands = () => {
    map.clear();
    refresh();
  };

  return {
    actions,
    registerCommand,
    registerCommands,
    unregisterCommand,
    clearCommands,
  };
}

export function provideDqCommandRegistry(registry: DqCommandRegistry): DqCommandRegistry {
  provide(registryKey, registry);
  return registry;
}

export function useDqCommandRegistry(): DqCommandRegistry | null {
  return inject(registryKey, null);
}
