import { reactive } from 'vue';
import type { ToastItem, ToastType } from './types';

export const toastState = reactive({
  items: [] as ToastItem[],
});

let nextId = 1;
const timers = new Map<number, ReturnType<typeof setTimeout>>();

export function pushToast(
  type: ToastType,
  message: string,
  opts?: { title?: string; duration?: number },
) {
  const id = nextId++;
  const duration = opts?.duration ?? (type === 'error' ? 6000 : 4000);
  const item: ToastItem = {
    id,
    type,
    message,
    title: opts?.title,
    duration,
  };
  toastState.items.push(item);
  const timer = setTimeout(() => dismissToast(id), duration);
  timers.set(id, timer);
  return id;
}

export function dismissToast(id: number) {
  const t = timers.get(id);
  if (t) {
    clearTimeout(t);
    timers.delete(id);
  }
  const idx = toastState.items.findIndex((i) => i.id === id);
  if (idx >= 0) toastState.items.splice(idx, 1);
}
