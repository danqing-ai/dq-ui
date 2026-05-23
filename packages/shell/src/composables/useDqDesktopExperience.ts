import { onBeforeUnmount, onMounted, ref, toValue, type MaybeRefOrGetter, type Ref } from 'vue';
import type { DqCommandAction } from '../components/commandPaletteTypes';

export interface UseDqDesktopExperienceOptions {
  onOpenPreferences?: () => void;
  onCloseCurrent?: () => void;
  onSelectTab?: (index: number) => void;
  enabled?: MaybeRefOrGetter<boolean>;
}

const IS_MAC =
  typeof navigator !== 'undefined' &&
  /(Mac|iPhone|iPad|iPod)/i.test(navigator.platform || navigator.userAgent || '');

function isEditableTarget(target: EventTarget | null): boolean {
  if (!(target instanceof HTMLElement)) return false;
  if (target.isContentEditable) return true;
  const tag = target.tagName.toLowerCase();
  return tag === 'input' || tag === 'textarea' || tag === 'select';
}

function matchShortcut(event: KeyboardEvent, shortcut: string): boolean {
  const parts = shortcut
    .toLowerCase()
    .split('+')
    .map((part) => part.trim())
    .filter(Boolean);
  if (parts.length === 0) return false;

  const key = parts[parts.length - 1];
  const needsMod = parts.includes('mod');
  const needsMeta = needsMod ? IS_MAC : parts.includes('meta');
  const needsCtrl = needsMod ? !IS_MAC : parts.includes('ctrl');
  const needsShift = parts.includes('shift');
  const needsAlt = parts.includes('alt') || parts.includes('option');

  if (event.metaKey !== needsMeta) return false;
  if (event.ctrlKey !== needsCtrl) return false;
  if (event.shiftKey !== needsShift) return false;
  if (event.altKey !== needsAlt) return false;

  return event.key.toLowerCase() === key;
}

export function useDqDesktopExperience(
  actions: MaybeRefOrGetter<DqCommandAction[]>,
  options: UseDqDesktopExperienceOptions = {},
): { commandPaletteOpen: Ref<boolean> } {
  const commandPaletteOpen = ref(false);

  const tabShortcutIndex = (event: KeyboardEvent): number | null => {
    const num = Number(event.key);
    if (!Number.isInteger(num) || num < 1 || num > 9) return null;
    if (!matchShortcut(event, `mod+${num}`)) return null;
    return num - 1;
  };

  const onKeydown = async (event: KeyboardEvent) => {
    if (toValue(options.enabled) === false) return;

    const items = toValue(actions);
    if (matchShortcut(event, 'mod+k')) {
      event.preventDefault();
      commandPaletteOpen.value = !commandPaletteOpen.value;
      return;
    }

    if (matchShortcut(event, 'mod+,')) {
      event.preventDefault();
      options.onOpenPreferences?.();
      return;
    }

    if (options.onCloseCurrent && matchShortcut(event, 'mod+w')) {
      event.preventDefault();
      options.onCloseCurrent();
      return;
    }

    const tabIndex = options.onSelectTab ? tabShortcutIndex(event) : null;
    if (tabIndex !== null) {
      event.preventDefault();
      options.onSelectTab?.(tabIndex);
      return;
    }

    if (isEditableTarget(event.target)) return;

    const matched = items.find((item) => item.shortcut && matchShortcut(event, item.shortcut));
    if (!matched || matched.disabled) return;
    event.preventDefault();
    await matched.run();
  };

  onMounted(() => window.addEventListener('keydown', onKeydown));
  onBeforeUnmount(() => window.removeEventListener('keydown', onKeydown));

  return { commandPaletteOpen };
}
