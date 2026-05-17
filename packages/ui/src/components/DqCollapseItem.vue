<script setup lang="ts">
import { computed, inject } from 'vue';
import { collapseContextKey } from './collapseContext';

const props = defineProps<{
  name: string;
  title?: string;
}>();

function useCollapseContext() {
  const ctx = inject(collapseContextKey);
  if (!ctx) {
    throw new Error('DqCollapseItem must be used inside DqCollapse');
  }
  return ctx;
}

const collapseCtx = useCollapseContext();

const open = computed(() => collapseCtx.isActive(props.name));

function onHeaderClick() {
  collapseCtx.toggle(props.name);
}
</script>

<template>
  <div class="dq-collapse-item" :class="{ 'is-active': open }">
    <button type="button" class="dq-collapse-item__header" @click="onHeaderClick">
      <span class="dq-collapse-item__title">
        <slot name="title">{{ title }}</slot>
      </span>
      <span class="dq-collapse-item__arrow" aria-hidden="true" />
    </button>
    <div v-show="open" class="dq-collapse-item__wrap">
      <div class="dq-collapse-item__content">
        <slot />
      </div>
    </div>
  </div>
</template>

