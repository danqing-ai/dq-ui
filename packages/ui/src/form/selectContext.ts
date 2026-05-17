import type { ComputedRef, InjectionKey, Ref } from 'vue';
import { inject, provide } from 'vue';

export interface DqSelectContext {
  filterQuery: Ref<string>;
  filterable: ComputedRef<boolean>;
}

export const DQ_SELECT_KEY: InjectionKey<DqSelectContext> = Symbol('dq-select');

export function provideDqSelect(ctx: DqSelectContext): void {
  provide(DQ_SELECT_KEY, ctx);
}

export function useDqSelect(): DqSelectContext | null {
  return inject(DQ_SELECT_KEY, null);
}
