<script setup lang="ts">
defineProps<{
  ariaLabel?: string;
}>();
</script>

<template>
  <aside
    class="dq-inspector-stack"
    :aria-label="ariaLabel"
  >
    <slot />
  </aside>
</template>
