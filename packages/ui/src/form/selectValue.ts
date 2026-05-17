import type { AcceptableValue } from 'reka-ui';

/** Internal SelectItem value for “clear / none” options (Reka forbids `value=""`). */
export const DQ_SELECT_NONE = '__dq_select_none__';

export function isEmptySelectValue(value: AcceptableValue | undefined | null): boolean {
  return value === '' || value == null;
}

export function toSelectItemValue(value: AcceptableValue): AcceptableValue {
  if (value === '') return DQ_SELECT_NONE;
  return value;
}

export function fromSelectRootValue(value: AcceptableValue | undefined): AcceptableValue | undefined {
  if (value === DQ_SELECT_NONE) return '';
  return value;
}

/** Map outward model → Reka SelectRoot (empty → unset so placeholder shows). */
export function toSelectRootValue(value: AcceptableValue | undefined | null): AcceptableValue | undefined {
  if (isEmptySelectValue(value)) return undefined;
  return value;
}
