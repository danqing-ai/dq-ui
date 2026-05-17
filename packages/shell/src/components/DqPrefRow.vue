<script setup lang="ts">
defineProps<{
  label?: string;
  noLabel?: boolean;
  stacked?: boolean;
}>();
</script>

<template>
  <div
    class="dq-pref-row"
    :class="[
      $attrs.class,
      { 'dq-pref-row--no-label': noLabel, 'dq-pref-row--stacked': stacked },
    ]"
  >
    <div
      v-if="!noLabel && (label || $slots.label)"
      class="dq-pref-row__label"
    >
      <slot name="label">{{ label }}</slot>
    </div>
    <div class="dq-pref-row__content">
      <slot />
    </div>
  </div>
</template>
