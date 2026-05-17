<script setup lang="ts">
import { TabsContent, TabsList, TabsRoot, TabsTrigger } from 'reka-ui';
import { computed } from 'vue';

const model = defineModel<string>({ required: true });

const props = defineProps<{
  /** Optional id prefix for a11y */
  id?: string;
}>();

const rootId = computed(() => props.id || 'dq-section-tabs');
</script>

<template>
  <TabsRoot
    v-model="model"
    :class="['dq-section-tabs', $attrs.class]"
    :orientation="'horizontal'"
  >
    <TabsList
      :class="'dq-section-tabs__list'"
      :aria-label="rootId"
    >
      <slot name="tabs" />
    </TabsList>
    <slot />
  </TabsRoot>
</template>
