<script setup lang="ts">
import { SliderRange, SliderRoot, SliderThumb, SliderTrack } from 'reka-ui';
import { computed, useAttrs } from 'vue';

defineOptions({ inheritAttrs: false });

const model = defineModel<number | number[] | undefined>();

const props = withDefaults(
  defineProps<{
    min?: number;
    max?: number;
    step?: number;
    disabled?: boolean;
  }>(),
  {
    min: 0,
    max: 100,
    step: 1,
  },
);

const attrs = useAttrs();

const sliderModel = computed({
  get(): number[] {
    const v = model.value;
    if (Array.isArray(v)) return v;
    if (v === undefined || v === null) return [props.min];
    return [v];
  },
  set(next: number[] | undefined) {
    if (!next?.length) {
      model.value = undefined;
      return;
    }
    model.value = next.length === 1 ? next[0] : next;
  },
});

const isDisabled = computed(() => props.disabled ?? Boolean(attrs.disabled));
</script>

<template>
  <SliderRoot
    v-model="sliderModel"
    :min="min"
    :max="max"
    :step="step"
    :disabled="isDisabled"
    class="dq-slider"
    v-bind="attrs"
  >
    <SliderTrack class="dq-slider__track">
      <SliderRange class="dq-slider__range" />
    </SliderTrack>
    <SliderThumb class="dq-slider__thumb" :aria-label="(attrs['aria-label'] as string) || undefined" />
  </SliderRoot>
</template>
