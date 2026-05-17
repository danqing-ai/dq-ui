export type ToastType = 'success' | 'error' | 'warning' | 'info';

export type ToastItem = {
  id: number;
  type: ToastType;
  message: string;
  title?: string;
  duration: number;
};

export type ConfirmOptions = {
  type?: 'warning' | 'info';
  confirmButtonText?: string;
  cancelButtonText?: string;
};

export type ConfirmRequest = {
  message: string;
  title: string;
  options: ConfirmOptions;
  resolve: () => void;
  reject: (reason: 'cancel') => void;
};
