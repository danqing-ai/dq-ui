<script setup lang="ts">
withDefaults(
  defineProps<{
    type?: 'success' | 'warning' | 'info' | 'error';
    title?: string;
    closable?: boolean;
    showIcon?: boolean;
  }>(),
  {
    type: 'info',
    closable: false,
    showIcon: false,
  },
);

const emit = defineEmits<{
  (e: 'close'): void;
}>();

function onClose() {
  emit('close');
}
</script>

<template>
  <div
    class="dq-alert"
    :class="[`dq-alert--${type}`, showIcon ? 'dq-alert--with-icon' : '']"
    role="alert"
  >
    <span v-if="showIcon" class="dq-alert__icon" aria-hidden="true">
      <slot name="icon" />
    </span>
    <div class="dq-alert__content">
      <div v-if="title || $slots.title" class="dq-alert__title">
        <slot name="title">{{ title }}</slot>
      </div>
      <div v-if="$slots.default" class="dq-alert__description">
        <slot />
      </div>
    </div>
    <button
      v-if="closable"
      type="button"
      class="dq-alert__close"
      aria-label="Close"
      @click="onClose"
    >
      ×
    </button>
  </div>
</template>
