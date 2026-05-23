<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref, watch } from 'vue';
import { DqDialog } from '@danqing/dq-ui';
import type { DqCommandAction } from './commandPaletteTypes';

const props = withDefaults(
  defineProps<{
    actions: DqCommandAction[];
    title?: string;
    placeholder?: string;
    emptyText?: string;
  }>(),
  {
    title: 'Command Palette',
    placeholder: 'Type a command...',
    emptyText: 'No matching commands',
  },
);

const open = defineModel<boolean>('open', { required: true });

const query = ref('');
const activeIndex = ref(0);
const inputRef = ref<HTMLInputElement | null>(null);

const normalizedQuery = computed(() => query.value.trim().toLowerCase());

const IS_MAC =
  typeof navigator !== 'undefined' &&
  /(Mac|iPhone|iPad|iPod)/i.test(navigator.platform || navigator.userAgent || '');

const filteredActions = computed(() => {
  if (!normalizedQuery.value) return props.actions;
  return props.actions.filter((action) => {
    const fields = [action.title, action.description ?? '', ...(action.keywords ?? [])]
      .join(' ')
      .toLowerCase();
    return fields.includes(normalizedQuery.value);
  });
});

function shortcutLabel(shortcut?: string): string {
  if (!shortcut) return '';
  const mapMac: Record<string, string> = {
    mod: '⌘',
    shift: '⇧',
    alt: '⌥',
    option: '⌥',
    ctrl: '⌃',
    meta: '⌘',
  };
  const mapWin: Record<string, string> = {
    mod: 'Ctrl',
    shift: 'Shift',
    alt: 'Alt',
    option: 'Alt',
    ctrl: 'Ctrl',
    meta: 'Meta',
  };
  const mapper = IS_MAC ? mapMac : mapWin;
  return shortcut
    .split('+')
    .map((part) => {
      const key = part.trim().toLowerCase();
      return mapper[key] ?? part.trim().toUpperCase();
    })
    .join(IS_MAC ? '' : '+');
}

watch(
  () => open.value,
  (isOpen) => {
    if (isOpen) {
      query.value = '';
      activeIndex.value = 0;
      requestAnimationFrame(() => inputRef.value?.focus());
      return;
    }
    query.value = '';
    activeIndex.value = 0;
  },
);

watch(
  () => filteredActions.value.length,
  (size) => {
    if (size === 0) {
      activeIndex.value = 0;
      return;
    }
    if (activeIndex.value >= size) activeIndex.value = size - 1;
  },
);

async function triggerAction(action: DqCommandAction) {
  if (action.disabled) return;
  await action.run();
  open.value = false;
}

function moveActive(step: number) {
  const size = filteredActions.value.length;
  if (size === 0) return;
  activeIndex.value = (activeIndex.value + step + size) % size;
}

async function onPaletteKeydown(event: KeyboardEvent) {
  if (!open.value) return;
  if (event.key === 'ArrowDown') {
    event.preventDefault();
    moveActive(1);
    return;
  }
  if (event.key === 'ArrowUp') {
    event.preventDefault();
    moveActive(-1);
    return;
  }
  if (event.key === 'Enter') {
    const active = filteredActions.value[activeIndex.value];
    if (!active) return;
    event.preventDefault();
    await triggerAction(active);
    return;
  }
  if (event.key === 'Escape') {
    event.preventDefault();
    open.value = false;
  }
}

onMounted(() => window.addEventListener('keydown', onPaletteKeydown));
onBeforeUnmount(() => window.removeEventListener('keydown', onPaletteKeydown));
</script>

<template>
  <DqDialog v-model:open="open" :title="title" width="min(680px, 92vw)">
    <div class="dq-command-palette">
      <input
        ref="inputRef"
        v-model="query"
        class="dq-command-palette__input"
        type="text"
        :placeholder="placeholder"
        aria-label="Search commands"
      />

      <div class="dq-command-palette__list" role="listbox" aria-label="Command list">
        <button
          v-for="(action, index) in filteredActions"
          :key="action.id"
          class="dq-command-palette__item"
          :class="{ 'is-active': index === activeIndex }"
          type="button"
          role="option"
          :aria-selected="index === activeIndex"
          :disabled="action.disabled"
          @mouseenter="activeIndex = index"
          @click="triggerAction(action)"
        >
          <span class="dq-command-palette__item-main">
            <span class="dq-command-palette__title-row">
              <span class="dq-command-palette__title">{{ action.title }}</span>
              <span v-if="action.group" class="dq-command-palette__group">{{ action.group }}</span>
            </span>
            <span v-if="action.description" class="dq-command-palette__desc">{{ action.description }}</span>
          </span>
          <kbd v-if="action.shortcut" class="dq-command-palette__kbd">
            {{ shortcutLabel(action.shortcut) }}
          </kbd>
        </button>

        <p v-if="filteredActions.length === 0" class="dq-command-palette__empty">
          {{ emptyText }}
        </p>
      </div>
    </div>
  </DqDialog>
</template>
