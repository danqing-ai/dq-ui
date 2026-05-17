<script setup lang="ts">
import { provide } from 'vue';
import { collapseContextKey, type CollapseContext } from './collapseContext';

const model = defineModel<string[]>({ default: () => [] });

const ctx: CollapseContext = {
  activeNames: model,
  toggle(name: string) {
    const list = [...model.value];
    const i = list.indexOf(name);
    if (i >= 0) {
      list.splice(i, 1);
    } else {
      list.push(name);
    }
    model.value = list;
  },
  isActive(name: string) {
    return model.value.includes(name);
  },
};

provide(collapseContextKey, ctx);
</script>

<template>
  <div class="dq-collapse" v-bind="$attrs">
    <slot />
  </div>
</template>
