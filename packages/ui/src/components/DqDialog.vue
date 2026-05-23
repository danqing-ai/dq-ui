<script setup lang="ts">
import { computed } from 'vue';
import { X } from 'lucide-vue-next';
import {
  DialogClose,
  DialogContent,
  DialogOverlay,
  DialogPortal,
  DialogRoot,
  DialogTitle,
} from 'reka-ui';

const props = withDefaults(
  defineProps<{
    title?: string;
    width?: string;
    center?: boolean;
    destroyOnClose?: boolean;
    closable?: boolean;
  }>(),
  {
    width: '500px',
    center: false,
    destroyOnClose: false,
    closable: true,
  },
);

function blockDismiss(event: Event) {
  if (!props.closable) {
    event.preventDefault();
  }
}

const open = defineModel<boolean>('open', { required: true });
const titleId = `dq-dialog-title-${Math.random().toString(36).slice(2, 8)}`;

const contentStyle = computed(() => ({
  width: props.width,
  maxWidth: 'min(96vw, 100%)',
}));
</script>

<template>
  <DialogRoot v-model:open="open">
    <DialogPortal>
      <DialogOverlay class="dq-dialog-overlay" />
      <DialogContent
        class="dq-dialog-content"
        :class="{ 'dq-dialog-content--center': center }"
        :style="contentStyle"
        :aria-modal="true"
        :aria-labelledby="title ? titleId : undefined"
        @interact-outside="blockDismiss"
        @escape-key-down="blockDismiss"
      >
        <header v-if="title || $slots.header" class="dq-dialog-header">
          <DialogTitle v-if="title" :id="titleId" class="dq-dialog-title">
            {{ title }}
          </DialogTitle>
          <slot name="header" />
          <DialogClose v-if="closable" class="dq-dialog-close" aria-label="Close">
            <X :size="16" stroke-width="2" aria-hidden="true" />
          </DialogClose>
        </header>

        <div v-if="destroyOnClose ? open : true" class="dq-dialog-body">
          <slot />
        </div>

        <footer v-if="$slots.footer" class="dq-dialog-footer">
          <slot name="footer" />
        </footer>
      </DialogContent>
    </DialogPortal>
  </DialogRoot>
</template>
