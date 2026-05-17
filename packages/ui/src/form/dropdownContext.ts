import type { InjectionKey } from 'vue';
import { inject, provide } from 'vue';

export interface DqDropdownContext {
  onCommand: (command: string) => void;
}

export const DQ_DROPDOWN_KEY: InjectionKey<DqDropdownContext> = Symbol('dq-dropdown');

export function provideDqDropdown(onCommand: (command: string) => void): void {
  provide(DQ_DROPDOWN_KEY, { onCommand });
}

export function useDqDropdown(): DqDropdownContext | null {
  return inject(DQ_DROPDOWN_KEY, null);
}
