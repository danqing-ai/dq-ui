import { reactive } from 'vue';
import type { ConfirmOptions, ConfirmRequest } from './types';

export const confirmState = reactive({
  current: null as ConfirmRequest | null,
});

export function showConfirm(
  message: string,
  title: string,
  options: ConfirmOptions,
): Promise<void> {
  return new Promise((resolve, reject) => {
    confirmState.current = {
      message,
      title,
      options,
      resolve: () => {
        confirmState.current = null;
        resolve();
      },
      reject: (reason) => {
        confirmState.current = null;
        reject(reason);
      },
    };
  });
}

export function confirmCurrent() {
  confirmState.current?.resolve();
}

export function cancelConfirm() {
  confirmState.current?.reject('cancel');
}
