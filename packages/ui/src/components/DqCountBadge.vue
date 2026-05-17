<script setup lang="ts">
import { computed } from 'vue';

const props = withDefaults(
  defineProps<{
    value?: number | string;
    hidden?: boolean;
    max?: number;
  }>(),
  {
    hidden: false,
    max: 99,
  },
);

const displayValue = computed(() => {
  if (props.hidden || props.value === undefined || props.value === null || props.value === '') {
    return '';
  }
  const n = Number(props.value);
  if (!Number.isNaN(n) && Number.isFinite(n) && n > props.max) {
    return `${props.max}+`;
  }
  return String(props.value);
});
</script>

<template>
  <span class="dq-count-badge">
    <slot />
    <sup v-if="displayValue" class="dq-count-badge__value">{{ displayValue }}</sup>
  </span>
</template>
