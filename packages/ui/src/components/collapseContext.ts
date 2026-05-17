import type { InjectionKey, Ref } from 'vue';

export type CollapseContext = {
  activeNames: Ref<string[]>;
  toggle: (name: string) => void;
  isActive: (name: string) => boolean;
};

export const collapseContextKey: InjectionKey<CollapseContext> = Symbol('dqCollapse');
