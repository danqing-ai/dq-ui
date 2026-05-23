<script setup lang="ts">
import { computed, watch } from 'vue';
import DqCommandPalette from './DqCommandPalette.vue';
import type { DqCommandAction } from './commandPaletteTypes';
import { useDqCommandActions } from '../composables/useDqCommandActions';
import { createDqCommandRegistry, provideDqCommandRegistry } from '../composables/useDqCommandRegistry';
import { useDqDesktopExperience } from '../composables/useDqDesktopExperience';
import { useDqRecentCommands } from '../composables/useDqRecentCommands';
import { useDqWindowActivity } from '../composables/useDqWindowActivity';

export interface DqDesktopHostExposed {
  openPalette: () => void;
  closePalette: () => void;
  togglePalette: () => void;
  registerCommand: (action: DqCommandAction) => () => void;
  registerCommands: (actions: DqCommandAction[]) => () => void;
  unregisterCommand: (id: string) => void;
  clearCommands: () => void;
}

const props = withDefaults(
  defineProps<{
    actions?: DqCommandAction[];
    title?: string;
    placeholder?: string;
    emptyText?: string;
    enabled?: boolean;
    includeDefaultCommands?: boolean;
    trackWindowActivity?: boolean;
    enableRecentCommands?: boolean;
    recentStorageKey?: string;
    recentLimit?: number;
    onOpenPreferences?: () => void;
    onReload?: () => void;
    onBack?: () => void;
    onForward?: () => void;
    onCloseCurrent?: () => void;
    onSelectTab?: (index: number) => void;
  }>(),
  {
    actions: () => [],
    title: 'Command Palette',
    placeholder: 'Type a command...',
    emptyText: 'No matching commands',
    enabled: true,
    includeDefaultCommands: true,
    trackWindowActivity: true,
    enableRecentCommands: true,
    recentStorageKey: 'dq-shell:recent-commands',
    recentLimit: 8,
    onOpenPreferences: undefined,
    onReload: undefined,
    onBack: undefined,
    onForward: undefined,
    onCloseCurrent: undefined,
    onSelectTab: undefined,
  },
);

const open = defineModel<boolean>('open', { default: false });
const canUseWindow = typeof window !== 'undefined';
const registry = provideDqCommandRegistry(createDqCommandRegistry());

const openPalette = () => {
  open.value = true;
};

const closePalette = () => {
  open.value = false;
};

const togglePalette = () => {
  open.value = !open.value;
};

const { actions } = useDqCommandActions([computed(() => props.actions), registry.actions], {
  includeDefaults: computed(() => props.includeDefaultCommands),
  defaults: {
    onOpenPreferences: () => props.onOpenPreferences?.(),
    onReload: () => {
      if (props.onReload) return props.onReload();
      if (canUseWindow) window.location.reload();
    },
    onBack: () => {
      if (props.onBack) return props.onBack();
      if (canUseWindow) window.history.back();
    },
    onForward: () => {
      if (props.onForward) return props.onForward();
      if (canUseWindow) window.history.forward();
    },
    onCloseCurrent: () => {
      if (props.onCloseCurrent) return props.onCloseCurrent();
      if (open.value) closePalette();
    },
    onSelectTab: (index) => props.onSelectTab?.(index),
  },
});

const { recentIds, markUsed } = useDqRecentCommands({
  storageKey: props.recentStorageKey,
  maxItems: props.recentLimit,
});

const rankedActions = computed<DqCommandAction[]>(() => {
  const order = new Map<string, number>(recentIds.value.map((id, index) => [id, index]));

  return [...actions.value]
    .sort((a, b) => {
      const ai = order.get(a.id);
      const bi = order.get(b.id);
      if (ai !== undefined && bi !== undefined) return ai - bi;
      if (ai !== undefined) return -1;
      if (bi !== undefined) return 1;
      const aOrder = a.order ?? 0;
      const bOrder = b.order ?? 0;
      if (aOrder !== bOrder) return aOrder - bOrder;
      return a.title.localeCompare(b.title);
    })
    .map((action) => {
      if (!props.enableRecentCommands) return action;
      const isRecent = order.has(action.id);
      return {
        ...action,
        group: isRecent ? 'Recent' : action.group,
      };
    });
});

const paletteActions = computed<DqCommandAction[]>(() =>
  rankedActions.value.map((action) => ({
    ...action,
    run: async () => {
      await action.run();
      if (props.enableRecentCommands) markUsed(action.id);
    },
  })),
);

const { commandPaletteOpen } = useDqDesktopExperience(paletteActions, {
  enabled: computed(() => props.enabled),
  onOpenPreferences: () => props.onOpenPreferences?.(),
  onCloseCurrent: () => {
    if (props.onCloseCurrent) return props.onCloseCurrent();
    if (commandPaletteOpen.value) closePalette();
  },
  onSelectTab: (index) => props.onSelectTab?.(index),
});

watch(
  () => open.value,
  (value) => {
    if (value === commandPaletteOpen.value) return;
    commandPaletteOpen.value = value;
  },
);

watch(
  () => commandPaletteOpen.value,
  (value) => {
    if (value === open.value) return;
    open.value = value;
  },
);

useDqWindowActivity({
  enabled: computed(() => props.enabled && props.trackWindowActivity),
});

defineExpose<DqDesktopHostExposed>({
  openPalette,
  closePalette,
  togglePalette,
  registerCommand: registry.registerCommand,
  registerCommands: registry.registerCommands,
  unregisterCommand: registry.unregisterCommand,
  clearCommands: registry.clearCommands,
});
</script>

<template>
  <slot
    :open-palette="openPalette"
    :close-palette="closePalette"
    :toggle-palette="togglePalette"
    :register-command="registry.registerCommand"
    :register-commands="registry.registerCommands"
    :unregister-command="registry.unregisterCommand"
    :clear-commands="registry.clearCommands"
  />
  <DqCommandPalette
    v-if="enabled"
    v-model:open="commandPaletteOpen"
    :actions="paletteActions"
    :title="title"
    :placeholder="placeholder"
    :empty-text="emptyText"
  />
</template>
