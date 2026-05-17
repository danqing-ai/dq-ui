<script setup lang="ts">
defineProps<{
  labelWidth?: string;
}>();
</script>

<template>
  <div
    class="dq-pref-form"
    :style="labelWidth ? { '--dq-pref-label-width': labelWidth } : undefined"
  >
    <slot />
  </div>
</template>
