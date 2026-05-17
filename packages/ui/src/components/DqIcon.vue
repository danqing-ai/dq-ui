<script setup lang="ts">
import { computed } from 'vue';

const props = withDefaults(
  defineProps<{
    size?: number | string;
    color?: string;
  }>(),
  {},
);

const style = computed(() => {
  const s: Record<string, string> = {};
  if (props.size != null) {
    const n = Number(props.size);
    s.width = Number.isFinite(n) ? `${n}px` : String(props.size);
    s.height = s.width;
    s.fontSize = s.width;
  }
  if (props.color) s.color = props.color;
  return s;
});
</script>

<template>
  <span class="dq-icon" :style="style" aria-hidden="true">
    <slot />
  </span>
</template>
