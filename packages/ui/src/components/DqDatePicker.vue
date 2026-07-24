<script setup lang="ts">
import { computed, useAttrs } from 'vue';

defineOptions({ inheritAttrs: false });

const model = defineModel<Date[] | null>();

const props = withDefaults(
  defineProps<{
    type?: string;
    startPlaceholder?: string;
    endPlaceholder?: string;
    size?: 'sm' | 'large';
    disabled?: boolean;
  }>(),
  { type: 'date' },
);

const attrs = useAttrs();

const isRange = computed(() => props.type === 'daterange' || attrs.type === 'daterange');

const sizeClass = computed(() => {
  const s = props.size ?? (attrs.size as string | undefined);
  if (s === 'sm') return 'dq-date-picker--sm';
  if (s === 'large') return 'dq-date-picker--lg';
  return '';
});

const isDisabled = computed(() => props.disabled ?? Boolean(attrs.disabled));

const startPlaceholderText = computed(
  () => props.startPlaceholder ?? (attrs['start-placeholder'] as string) ?? '',
);
const endPlaceholderText = computed(
  () => props.endPlaceholder ?? (attrs['end-placeholder'] as string) ?? '',
);

function toInputValue(date: Date): string {
  const y = date.getFullYear();
  const m = String(date.getMonth() + 1).padStart(2, '0');
  const d = String(date.getDate()).padStart(2, '0');
  return `${y}-${m}-${d}`;
}

function fromInputValue(value: string): Date | null {
  if (!value) return null;
  const [y, m, d] = value.split('-').map((n) => Number(n));
  if (!y || !m || !d) return null;
  return new Date(y, m - 1, d);
}

const startValue = computed({
  get(): string {
    return model.value?.[0] ? toInputValue(model.value[0]) : '';
  },
  set(raw: string) {
    syncRange(raw, endValue.value);
  },
});

const endValue = computed({
  get(): string {
    return model.value?.[1] ? toInputValue(model.value[1]) : '';
  },
  set(raw: string) {
    syncRange(startValue.value, raw);
  },
});

const singleValue = computed({
  get(): string {
    return model.value?.[0] ? toInputValue(model.value[0]) : '';
  },
  set(raw: string) {
    const parsed = fromInputValue(raw);
    model.value = parsed ? [parsed] : null;
  },
});

function syncRange(startRaw: string, endRaw: string) {
  const start = fromInputValue(startRaw);
  const end = fromInputValue(endRaw);
  if (!start && !end) {
    model.value = null;
    return;
  }
  if (start && end) {
    model.value = start <= end ? [start, end] : [end, start];
    return;
  }
  model.value = null;
}
</script>

<template>
  <div
    v-if="isRange"
    class="dq-date-picker dq-date-picker--range"
    :class="sizeClass"
    v-bind="attrs"
  >
    <input
      v-model="startValue"
      type="date"
      class="dq-date-picker__input"
      :disabled="isDisabled"
      :aria-label="startPlaceholderText || 'Start date'"
    />
    <span class="dq-date-picker__sep" aria-hidden="true">–</span>
    <input
      v-model="endValue"
      type="date"
      class="dq-date-picker__input"
      :disabled="isDisabled"
      :aria-label="endPlaceholderText || 'End date'"
    />
  </div>
  <input
    v-else
    v-model="singleValue"
    type="date"
    class="dq-date-picker dq-date-picker__input"
    :class="sizeClass"
    :disabled="isDisabled"
    v-bind="attrs"
  />
</template>
