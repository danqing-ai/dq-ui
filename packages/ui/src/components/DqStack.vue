<script setup lang="ts">
import { computed } from 'vue';

const props = withDefaults(
  defineProps<{
    gap?: number | string;
    wrap?: boolean;
    justify?: 'start' | 'end' | 'center' | 'between';
    align?: 'start' | 'end' | 'center' | 'stretch';
    inline?: boolean;
  }>(),
  {
    gap: 8,
    wrap: false,
    justify: 'start',
    align: 'center',
    inline: false,
  },
);

const style = computed(() => ({
  gap: typeof props.gap === 'number' ? `${props.gap}px` : props.gap,
}));
</script>

<template>
  <div
    class="dq-stack"
    :class="[
      wrap ? 'dq-stack--wrap' : '',
      inline ? 'dq-stack--inline' : '',
      justify ? `dq-stack--justify-${justify}` : '',
      align ? `dq-stack--align-${align}` : '',
    ]"
    :style="style"
  >
    <slot />
  </div>
</template>
