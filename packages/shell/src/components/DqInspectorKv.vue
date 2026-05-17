<script setup lang="ts">
defineProps<{
  label: string;
  value: string;
}>();
</script>

<template>
  <div class="dq-inspector-kv">
    <span class="dq-inspector-kv__label">{{ label }}</span>
    <span class="dq-inspector-kv__value">{{ value }}</span>
  </div>
</template>
