import { toValue, watchEffect, type MaybeRefOrGetter } from 'vue';
import type { DqCommandAction } from '../components/commandPaletteTypes';
import { useDqCommandRegistry } from './useDqCommandRegistry';

export interface UseDqRegisterCommandsOptions {
  enabled?: MaybeRefOrGetter<boolean>;
}

/**
 * Register commands for the current feature/module lifecycle.
 * Commands are auto-registered on mount and auto-unregistered on dispose.
 */
export function useDqRegisterCommands(
  actions: MaybeRefOrGetter<DqCommandAction[]>,
  options: UseDqRegisterCommandsOptions = {},
) {
  const registry = useDqCommandRegistry();

  watchEffect((onCleanup) => {
    if (!registry) return;
    if (toValue(options.enabled) === false) return;
    const list = toValue(actions);
    if (!Array.isArray(list) || list.length === 0) return;
    const unregister = registry.registerCommands(list);
    onCleanup(() => unregister());
  });
}
