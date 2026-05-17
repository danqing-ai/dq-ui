<script setup lang="ts">
import { computed } from 'vue';

export type DqSegmentOption = {
  label: string;
  value: string | number;
  disabled?: boolean;
};

const props = withDefaults(
  defineProps<{
    options: Array<DqSegmentOption | string | number>;
    block?: boolean;
  }>(),
  {
    block: false,
  },
);

const model = defineModel<string | number>({ required: true });

const normalized = computed(() =>
  props.options.map((item) => {
    if (typeof item === 'object' && item !== null && 'value' in item) {
      return item as DqSegmentOption;
    }
    return { label: String(item), value: item as string | number };
  }),
);

const rootClass = computed(() => [
  'dq-segmented',
  props.block ? 'dq-segmented--block' : '',
]);
</script>

<template>
  <div :class="rootClass" role="tablist">
    <button
      v-for="opt in normalized"
      :key="String(opt.value)"
      type="button"
      role="tab"
      class="dq-segmented__item"
      :class="{ 'is-active': model === opt.value }"
      :aria-selected="model === opt.value"
      :disabled="opt.disabled"
      @click="model = opt.value"
    >
      {{ opt.label }}
    </button>
  </div>
</template>
