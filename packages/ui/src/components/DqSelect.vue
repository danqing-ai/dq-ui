<script setup lang="ts">
import { ChevronDown, X } from 'lucide-vue-next';
import {
  SelectContent,
  SelectIcon,
  SelectPortal,
  SelectRoot,
  SelectTrigger,
  SelectValue,
  SelectViewport,
  type AcceptableValue,
} from 'reka-ui';
import { computed, ref, useAttrs, watch } from 'vue';
import { provideDqSelect } from '../form/selectContext';
import { fromSelectRootValue, isEmptySelectValue, toSelectRootValue } from '../form/selectValue';

defineOptions({ inheritAttrs: false });

const model = defineModel<AcceptableValue | AcceptableValue[] | undefined>();

const rootModel = computed({
  get(): AcceptableValue | AcceptableValue[] | undefined {
    const v = model.value;
    if (Array.isArray(v)) {
      return v.map((item) => toSelectRootValue(item) as AcceptableValue);
    }
    return toSelectRootValue(v);
  },
  set(next: AcceptableValue | AcceptableValue[] | undefined) {
    if (Array.isArray(next)) {
      model.value = next.map((item) => fromSelectRootValue(item) as AcceptableValue);
      return;
    }
    model.value = fromSelectRootValue(next);
  },
});

const props = withDefaults(
  defineProps<{
    placeholder?: string;
    disabled?: boolean;
    multiple?: boolean;
    filterable?: boolean;
    clearable?: boolean;
    /** `small` / `sm` for compact chips (composer, toolbars); `large` for forms */
    size?: 'small' | 'sm' | 'large';
  }>(),
  {},
);

const emit = defineEmits<{
  change: [value: AcceptableValue | AcceptableValue[] | undefined];
}>();

const attrs = useAttrs();
const filterQuery = ref('');

const isFilterable = computed(() => props.filterable ?? Boolean(attrs.filterable));

provideDqSelect({
  filterQuery,
  filterable: isFilterable,
});

watch(model, (value) => {
  emit('change', value);
});

const sizeClass = computed(() => {
  const s = props.size ?? (attrs.size as string | undefined);
  if (s === 'small' || s === 'sm') return 'dq-select--sm';
  if (s === 'large') return 'dq-select--lg';
  return '';
});

const isDisabled = computed(() => props.disabled ?? Boolean(attrs.disabled));
const isMultiple = computed(() => props.multiple ?? Boolean(attrs.multiple));
const isClearable = computed(() => props.clearable ?? Boolean(attrs.clearable));

const hasValue = computed(() => {
  const v = model.value;
  if (Array.isArray(v)) return v.length > 0;
  return !isEmptySelectValue(v);
});

function clearValue(event: MouseEvent) {
  event.preventDefault();
  event.stopPropagation();
  model.value = isMultiple.value ? [] : undefined;
  emit('change', model.value);
}

</script>

<template>
  <SelectRoot
    v-model="rootModel"
    :multiple="isMultiple"
    :disabled="isDisabled"
    class="dq-select"
    :class="sizeClass"
    v-bind="attrs"
  >
    <SelectTrigger class="dq-select__trigger">
      <SelectValue v-slot="valueSlot" class="dq-select__value" :placeholder="placeholder">
        <slot name="value" v-bind="valueSlot">
          {{ valueSlot.selectedLabel?.length ? valueSlot.selectedLabel.join(', ') : placeholder }}
        </slot>
      </SelectValue>
      <button
        v-if="isClearable && hasValue && !isDisabled"
        type="button"
        class="dq-select__clear"
        :aria-label="'Clear'"
        @pointerdown.stop.prevent
        @click="clearValue"
      >
        <X aria-hidden="true" />
      </button>
      <SelectIcon class="dq-select__chevron" as-child>
        <ChevronDown aria-hidden="true" />
      </SelectIcon>
    </SelectTrigger>

    <SelectPortal>
      <SelectContent class="dq-select__content" position="popper" :side-offset="4">
        <input
          v-if="isFilterable"
          v-model="filterQuery"
          type="search"
          class="dq-select__filter"
          autocomplete="off"
          @keydown.stop
          @click.stop
        />
        <SelectViewport class="dq-select__viewport">
          <slot />
        </SelectViewport>
      </SelectContent>
    </SelectPortal>
  </SelectRoot>
</template>
