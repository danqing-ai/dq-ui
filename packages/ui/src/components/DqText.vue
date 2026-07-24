<script setup lang="ts">
withDefaults(
  defineProps<{
    type?: 'primary' | 'success' | 'warning' | 'danger' | 'info' | '';
    size?: 'sm' | 'default' | 'large';
  }>(),
  {
    type: '',
    size: 'default',
  },
);
</script>

<template>
  <span
    class="dq-text"
    :class="[
      type ? `dq-text--${type}` : '',
      size !== 'default' ? `dq-text--${size}` : '',
    ]"
  >
    <slot />
  </span>
</template>
