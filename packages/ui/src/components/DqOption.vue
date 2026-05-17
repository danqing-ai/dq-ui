<script setup lang="ts">
import { SelectItem, SelectItemIndicator, SelectItemText, type AcceptableValue } from 'reka-ui';
import { Check } from 'lucide-vue-next';
import { computed } from 'vue';
import { useDqSelect } from '../form/selectContext';
import { toSelectItemValue } from '../form/selectValue';

const props = defineProps<{
  label?: string;
  value: AcceptableValue;
  disabled?: boolean;
}>();

const select = useDqSelect();

const itemValue = computed(() => toSelectItemValue(props.value));

const displayLabel = computed(() => String(props.label ?? props.value ?? ''));

const visible = computed(() => {
  if (!select?.filterable.value || !select.filterQuery.value.trim()) return true;
  const q = select.filterQuery.value.trim().toLowerCase();
  const hay = String(props.label ?? props.value ?? '').toLowerCase();
  return hay.includes(q);
});
</script>

<template>
  <SelectItem
    v-show="visible"
    :value="itemValue"
    :disabled="disabled"
    :text-value="displayLabel"
    class="dq-select__option"
  >
    <SelectItemText class="dq-select__option-text">{{ displayLabel }}</SelectItemText>
    <span v-if="$slots.default" class="dq-select__option-suffix">
      <slot />
    </span>
    <SelectItemIndicator class="dq-select__option-check">
      <Check aria-hidden="true" />
    </SelectItemIndicator>
  </SelectItem>
</template>
