<script setup lang="ts">
import { computed, useAttrs } from 'vue';

defineOptions({ inheritAttrs: false });

const props = withDefaults(
  defineProps<{
    type?: 'default' | 'primary' | 'text' | 'danger';
    size?: 'default' | 'sm';
    disabled?: boolean;
    loading?: boolean;
    round?: boolean;
    plain?: boolean;
    circle?: boolean;
  }>(),
  {
    type: 'default',
    size: 'default',
    disabled: false,
    loading: false,
    round: false,
    plain: false,
    circle: false,
  },
);

const attrs = useAttrs();

const cls = computed(() => {
  const attrClass = attrs.class;
  const classStr = typeof attrClass === 'string' ? attrClass : '';
  const legacyPlain = props.plain || classStr.includes('is-plain');
  const legacyCircle = props.circle || classStr.includes('is-circle');
  const legacyRound = props.round || legacyCircle;
  return [
    'dq-btn',
    `dq-btn--${props.type}`,
    props.size === 'sm' ? 'dq-btn--sm' : '',
    legacyPlain ? 'dq-btn--plain is-plain' : '',
    legacyRound ? 'dq-btn--round' : '',
    legacyCircle ? 'is-circle' : '',
    attrClass,
  ];
});
</script>

<template>
  <button
    type="button"
    :class="cls"
    :disabled="disabled || loading"
    v-bind="{ ...attrs, class: undefined }"
  >
    <slot />
  </button>
</template>
