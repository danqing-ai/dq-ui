<script setup lang="ts">
import { CheckboxGroupRoot, type AcceptableValue } from 'reka-ui';

const model = defineModel<AcceptableValue[] | undefined>();
</script>

<template>
  <CheckboxGroupRoot v-model="model" class="dq-checkbox-group" v-bind="$attrs">
    <slot />
  </CheckboxGroupRoot>
</template>
