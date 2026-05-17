<script setup lang="ts">
withDefaults(
  defineProps<{
    variant?: 'success' | 'warn';
    title?: string;
  }>(),
  { variant: 'success' },
);
</script>

<template>
  <div
    class="dq-inspector-callout"
    :class="`dq-inspector-callout--${variant}`"
  >
    <div
      v-if="title"
      class="dq-inspector-callout__title"
    >
      {{ title }}
    </div>
    <div
      v-if="$slots.default"
      class="dq-inspector-callout__body"
    >
      <slot />
    </div>
  </div>
</template>
