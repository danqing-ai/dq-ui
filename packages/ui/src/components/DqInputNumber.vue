<script setup lang="ts">
import { Minus, Plus } from 'lucide-vue-next';
import {
  NumberFieldDecrement,
  NumberFieldIncrement,
  NumberFieldInput,
  NumberFieldRoot,
} from 'reka-ui';
import { computed, useAttrs } from 'vue';

defineOptions({ inheritAttrs: false });

const model = defineModel<number | undefined>();

const props = withDefaults(
  defineProps<{
    min?: number;
    max?: number;
    step?: number;
    precision?: number;
    disabled?: boolean;
    controlsPosition?: 'right' | '';
  }>(),
  { step: 1 },
);

const attrs = useAttrs();

const formatOptions = computed(() => {
  if (props.precision == null) return undefined;
  return {
    maximumFractionDigits: props.precision,
    minimumFractionDigits: 0,
  };
});

const controlsRight = computed(
  () => props.controlsPosition === 'right' || attrs['controls-position'] === 'right',
);

const controlsClass = computed(() =>
  controlsRight.value ? 'dq-input-number--controls-right' : '',
);

const isDisabled = computed(() => props.disabled ?? Boolean(attrs.disabled));
</script>

<template>
  <NumberFieldRoot
    v-model="model"
    :min="min"
    :max="max"
    :step="step"
    :disabled="isDisabled"
    :format-options="formatOptions"
    class="dq-input-number"
    :class="controlsClass"
    v-bind="attrs"
  >
    <NumberFieldDecrement
      class="dq-input-number__btn dq-input-number__btn--dec"
      aria-label="Decrease"
    >
      <Minus aria-hidden="true" />
    </NumberFieldDecrement>
    <NumberFieldInput class="dq-input-number__input" />
    <NumberFieldIncrement
      class="dq-input-number__btn dq-input-number__btn--inc"
      aria-label="Increase"
    >
      <Plus aria-hidden="true" />
    </NumberFieldIncrement>
  </NumberFieldRoot>
</template>
