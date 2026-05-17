<script setup lang="ts">
withDefaults(
  defineProps<{
    type?: 'primary' | 'success' | 'warning' | 'danger' | 'info' | '';
    size?: 'small' | 'default' | 'large';
    effect?: 'dark' | 'light' | 'plain';
  }>(),
  {
    type: '',
    size: 'default',
    effect: 'light',
  },
);
</script>

<template>
  <span
    class="dq-tag"
    :class="[
      type ? `dq-tag--${type}` : '',
      size !== 'default' ? `dq-tag--${size}` : '',
      effect ? `dq-tag--${effect}` : '',
    ]"
  >
    <slot />
  </span>
</template>
