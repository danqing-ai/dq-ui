import { pushToast } from './toastState';
import type { ToastType } from './types';

function show(type: ToastType, message: string, duration?: number) {
  return pushToast(type, message, { duration });
}

export const toast = {
  success(message: string, duration?: number) {
    return show('success', message, duration);
  },
  error(message: string, duration?: number) {
    return show('error', message, duration);
  },
  warning(message: string, duration?: number) {
    return show('warning', message, duration);
  },
  info(message: string, duration?: number) {
    return show('info', message, duration);
  },
  /** Global error handlers (title + longer duration). */
  notify(opts: { title: string; message: string; type?: ToastType; duration?: number }) {
    return pushToast(opts.type ?? 'error', opts.message, {
      title: opts.title,
      duration: opts.duration ?? 6000,
    });
  },
};
