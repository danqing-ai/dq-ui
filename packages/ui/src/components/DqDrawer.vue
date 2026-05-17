<script setup lang="ts">
import { computed, useAttrs } from 'vue';
import {
  DialogClose,
  DialogContent,
  DialogOverlay,
  DialogPortal,
  DialogRoot,
  DialogTitle,
} from 'reka-ui';

defineOptions({ inheritAttrs: false });

const props = withDefaults(
  defineProps<{
    title?: string;
    size?: string;
    direction?: 'rtl' | 'ltr';
    closable?: boolean;
  }>(),
  {
    size: 'min(360px, 92vw)',
    direction: 'rtl',
    closable: true,
  },
);

const open = defineModel<boolean>('open', { required: true });
const attrs = useAttrs();

const panelStyle = computed(() => ({
  width: props.size,
  maxWidth: '100%',
}));

function blockDismiss(event: Event) {
  if (!props.closable) {
    event.preventDefault();
  }
}
</script>

<template>
  <DialogRoot v-model:open="open">
    <DialogPortal>
      <DialogOverlay class="dq-drawer-overlay" />
      <DialogContent
        class="dq-drawer-panel"
        :class="[`dq-drawer-panel--${direction}`, attrs.class]"
        :style="panelStyle"
        @interact-outside="blockDismiss"
        @escape-key-down="blockDismiss"
      >
        <header v-if="title || $slots.header" class="dq-drawer-header">
          <DialogTitle v-if="title" class="dq-drawer-title">
            {{ title }}
          </DialogTitle>
          <slot name="header" />
          <DialogClose v-if="closable" class="dq-drawer-close" aria-label="Close">×</DialogClose>
        </header>

        <div class="dq-drawer-body">
          <slot />
        </div>

        <footer v-if="$slots.footer" class="dq-drawer-footer">
          <slot name="footer" />
        </footer>
      </DialogContent>
    </DialogPortal>
  </DialogRoot>
</template>
