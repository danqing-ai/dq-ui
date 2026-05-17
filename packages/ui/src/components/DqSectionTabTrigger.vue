<script setup lang="ts">
import { TabsTrigger } from 'reka-ui';

defineProps<{
  name: string;
  label: string;
  disabled?: boolean;
}>();
</script>

<template>
  <TabsTrigger
    :value="name"
    :disabled="disabled"
    class="dq-section-tabs__trigger"
  >
    {{ label }}
  </TabsTrigger>
</template>
