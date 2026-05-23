import { computed, toValue, type MaybeRefOrGetter, type Ref } from 'vue';
import type { DqCommandAction } from '../components/commandPaletteTypes';

export interface DqDefaultCommandOptions {
  onOpenPreferences?: () => void;
  onReload?: () => void;
  onBack?: () => void;
  onForward?: () => void;
  onCloseCurrent?: () => void;
  onSelectTab?: (index: number) => void;
}

export interface UseDqCommandActionsOptions {
  includeDefaults?: MaybeRefOrGetter<boolean>;
  defaults?: DqDefaultCommandOptions;
}

function sortActions(a: DqCommandAction, b: DqCommandAction): number {
  const aOrder = a.order ?? 0;
  const bOrder = b.order ?? 0;
  if (aOrder !== bOrder) return aOrder - bOrder;
  return a.title.localeCompare(b.title);
}

export function createDqDefaultCommandActions(
  options: DqDefaultCommandOptions = {},
): DqCommandAction[] {
  const base: DqCommandAction[] = [
    {
      id: 'desktop.preferences',
      title: 'Open Preferences',
      description: 'Open app preferences panel',
      keywords: ['settings', 'preferences'],
      shortcut: 'mod+,',
      group: 'Application',
      order: 10,
      disabled: !options.onOpenPreferences,
      run: () => options.onOpenPreferences?.(),
    },
    {
      id: 'desktop.reload',
      title: 'Reload Window',
      description: 'Reload the current desktop window',
      keywords: ['reload', 'refresh'],
      shortcut: 'mod+r',
      group: 'Application',
      order: 20,
      disabled: !options.onReload,
      run: () => options.onReload?.(),
    },
    {
      id: 'desktop.back',
      title: 'Go Back',
      description: 'Navigate to previous page',
      keywords: ['history', 'back'],
      shortcut: 'mod+[',
      group: 'Navigation',
      order: 30,
      disabled: !options.onBack,
      run: () => options.onBack?.(),
    },
    {
      id: 'desktop.forward',
      title: 'Go Forward',
      description: 'Navigate to next page',
      keywords: ['history', 'forward'],
      shortcut: 'mod+]',
      group: 'Navigation',
      order: 40,
      disabled: !options.onForward,
      run: () => options.onForward?.(),
    },
    {
      id: 'desktop.close-current',
      title: 'Close Current',
      description: 'Close current tab or panel',
      keywords: ['close', 'tab', 'panel'],
      shortcut: 'mod+w',
      group: 'Navigation',
      order: 50,
      disabled: !options.onCloseCurrent,
      run: () => options.onCloseCurrent?.(),
    },
  ];

  const tabCommands = Array.from({ length: 9 }, (_, i) => {
    const tabNumber = i + 1;
    return {
      id: `desktop.switch-tab-${tabNumber}`,
      title: `Switch to Tab ${tabNumber}`,
      description: `Jump to tab ${tabNumber}`,
      keywords: ['tab', 'switch', String(tabNumber)],
      shortcut: `mod+${tabNumber}`,
      group: 'Navigation',
      order: 60 + i,
      disabled: !options.onSelectTab,
      run: () => options.onSelectTab?.(i),
    } satisfies DqCommandAction;
  });

  return [...base, ...tabCommands];
}

export function useDqCommandActions(
  sources: Array<MaybeRefOrGetter<DqCommandAction[]>>,
  options: UseDqCommandActionsOptions = {},
): { actions: Ref<DqCommandAction[]> } {
  const actions = computed(() => {
    const merged = new Map<string, DqCommandAction>();
    if (toValue(options.includeDefaults) !== false) {
      for (const action of createDqDefaultCommandActions(options.defaults)) {
        merged.set(action.id, action);
      }
    }
    for (const source of sources) {
      for (const action of toValue(source)) {
        merged.set(action.id, action);
      }
    }
    return [...merged.values()].sort(sortActions);
  });

  return { actions };
}
