export interface DqCommandAction {
  id: string;
  title: string;
  description?: string;
  keywords?: string[];
  shortcut?: string;
  disabled?: boolean;
  group?: string;
  order?: number;
  run: () => void | Promise<void>;
}
