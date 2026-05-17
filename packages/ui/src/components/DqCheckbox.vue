<script setup lang="ts">
import { Check } from 'lucide-vue-next';
import {
  CheckboxIndicator,
  CheckboxRoot,
  injectCheckboxGroupRootContext,
  type AcceptableValue,
} from 'reka-ui';
import { computed, useAttrs } from 'vue';

defineOptions({ inheritAttrs: false });

const model = defineModel<boolean | undefined>();

const props = withDefaults(
  defineProps<{
    /** Option value when used inside `DqCheckboxGroup` */
    label?: string | number | boolean;
    value?: string | number | boolean;
    disabled?: boolean;
    size?: 'small' | 'large';
  }>(),
  {},
);

const attrs = useAttrs();
const group = injectCheckboxGroupRootContext(null);
const inGroup = computed(() => group != null);

const itemValue = computed((): AcceptableValue => {
  if (props.label !== undefined && props.label !== true) return props.label as AcceptableValue;
  if (props.value !== undefined) return props.value as AcceptableValue;
  return 'on';
});

const sizeClass = computed(() => {
  const s = props.size ?? (attrs.size as string | undefined);
  if (s === 'small') return 'dq-checkbox--sm';
  if (s === 'large') return 'dq-checkbox--lg';
  return '';
});

const isDisabled = computed(() => props.disabled ?? Boolean(attrs.disabled));
</script>

<template>
  <label
    class="dq-checkbox"
    :class="[sizeClass, { 'dq-checkbox--disabled': isDisabled }]"
  >
    <CheckboxRoot
      v-if="inGroup"
      :value="itemValue"
      :disabled="isDisabled"
      class="dq-checkbox__control"
      v-bind="attrs"
    >
      <CheckboxIndicator class="dq-checkbox__indicator">
        <Check class="dq-checkbox__check" aria-hidden="true" />
      </CheckboxIndicator>
    </CheckboxRoot>
    <CheckboxRoot
      v-else
      v-model="model"
      :disabled="isDisabled"
      class="dq-checkbox__control"
      v-bind="attrs"
    >
      <CheckboxIndicator class="dq-checkbox__indicator">
        <Check class="dq-checkbox__check" aria-hidden="true" />
      </CheckboxIndicator>
    </CheckboxRoot>
    <span v-if="$slots.default" class="dq-checkbox__label">
      <slot />
    </span>
  </label>
</template>
