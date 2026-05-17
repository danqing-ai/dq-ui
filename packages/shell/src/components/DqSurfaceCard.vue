<script setup lang="ts">
defineOptions({ inheritAttrs: false });
</script>

<template>
  <div class="dq-surface-card" v-bind="$attrs">
    <header v-if="$slots.header" class="dq-surface-card__header">
      <slot name="header" />
    </header>
    <div class="dq-surface-card__body">
      <slot />
    </div>
  </div>
</template>
