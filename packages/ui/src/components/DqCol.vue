<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps<{
  span?: number;
  xs?: number;
  sm?: number;
  md?: number;
  lg?: number;
  xl?: number;
}>();

function pct(n: number) {
  return `${(n / 24) * 100}%`;
}

const baseSpan = computed(() => props.xs ?? props.span ?? 24);
const smSpan = computed(() => props.sm ?? baseSpan.value);
const mdSpan = computed(() => props.md ?? smSpan.value);
const lgSpan = computed(() => props.lg ?? mdSpan.value);
const xlSpan = computed(() => props.xl ?? lgSpan.value);

const colStyle = computed(() => ({
  '--dq-col-base': pct(baseSpan.value),
  '--dq-col-sm': pct(smSpan.value),
  '--dq-col-md': pct(mdSpan.value),
  '--dq-col-lg': pct(lgSpan.value),
  '--dq-col-xl': pct(xlSpan.value),
}));
</script>

<template>
  <div class="dq-col" :style="colStyle" v-bind="$attrs">
    <slot />
  </div>
</template>
