<script setup lang="ts">
import { computed, useAttrs } from 'vue';
import { SwitchRoot, SwitchThumb } from 'reka-ui';

defineOptions({ inheritAttrs: false });

const model = defineModel<boolean | string | number | undefined>();

const props = withDefaults(
  defineProps<{
    disabled?: boolean;
    size?: 'sm' | 'large';
  }>(),
  {},
);

const attrs = useAttrs();

const sizeClass = computed(() => {
  const s = props.size ?? (attrs.size as string | undefined);
  if (s === 'sm') return 'dq-switch--sm';
  if (s === 'large') return 'dq-switch--lg';
  return '';
});

const isDisabled = computed(() => props.disabled ?? Boolean(attrs.disabled));
</script>

<template>
  <SwitchRoot
    v-model="model"
    :disabled="isDisabled"
    class="dq-switch"
    :class="sizeClass"
    v-bind="attrs"
  >
    <SwitchThumb class="dq-switch__thumb" />
  </SwitchRoot>
</template>
