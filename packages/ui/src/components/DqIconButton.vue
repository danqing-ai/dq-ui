<script setup lang="ts">
import { computed } from 'vue';

const props = withDefaults(
  defineProps<{
    type?: 'default' | 'primary' | 'text' | 'danger';
    size?: 'default' | 'sm' | 'lg';
    disabled?: boolean;
    label?: string;
  }>(),
  {
    type: 'text',
    size: 'default',
    disabled: false,
  },
);

const cls = computed(() => [
  'dq-icon-btn',
  `dq-icon-btn--${props.type}`,
  props.size === 'sm' ? 'dq-icon-btn--sm' : '',
  props.size === 'lg' ? 'dq-icon-btn--lg' : '',
]);
</script>

<template>
  <button
    type="button"
    :class="cls"
    :disabled="disabled"
    :aria-label="label"
    :title="label"
  >
    <slot />
  </button>
</template>
