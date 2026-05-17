<script setup lang="ts">
import { TabsContent } from 'reka-ui';

defineProps<{
  name: string;
}>();
</script>

<template>
  <TabsContent
    :value="name"
    class="dq-section-tabs__content"
  >
    <slot />
  </TabsContent>
</template>
