<script setup lang="ts">
import { computed } from 'vue';

const props = withDefaults(
  defineProps<{
    percentage?: number;
    status?: '' | 'success' | 'exception' | 'warning';
    strokeWidth?: number;
    showText?: boolean;
  }>(),
  {
    percentage: 0,
    status: '',
    strokeWidth: 6,
    showText: true,
  },
);

const pct = computed(() => Math.min(100, Math.max(0, Number(props.percentage) || 0)));
</script>

<template>
  <div
    class="dq-progress"
    :class="[
      status ? `dq-progress--${status}` : '',
      showText ? 'dq-progress--with-text' : '',
    ]"
    role="progressbar"
    :aria-valuenow="pct"
    aria-valuemin="0"
    aria-valuemax="100"
  >
    <div class="dq-progress__track" :style="{ height: `${strokeWidth}px` }">
      <div class="dq-progress__bar" :style="{ width: `${pct}%` }" />
    </div>
    <span v-if="showText" class="dq-progress__text">{{ Math.round(pct) }}%</span>
  </div>
</template>


