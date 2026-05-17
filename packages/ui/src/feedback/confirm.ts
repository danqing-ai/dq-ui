import { showConfirm } from './confirmState';
import type { ConfirmOptions } from './types';

/**
 * Modal confirm — rejects with `'cancel'` when dismissed.
 */
export function confirm(
  message: string,
  title?: string,
  options?: ConfirmOptions,
): Promise<void> {
  return showConfirm(message, title ?? '', options ?? {});
}
