<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps<{
  gutter?: number;
}>();

const style = computed(() =>
  props.gutter != null ? { '--dq-row-gutter': `${props.gutter}px` } : undefined,
);
</script>

<template>
  <div class="dq-row" :style="style" v-bind="$attrs">
    <slot />
  </div>
</template>
