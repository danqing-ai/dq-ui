<script setup lang="ts">
import { computed } from 'vue';
import DqButton from './DqButton.vue';
import DqDialog from './DqDialog.vue';
import { cancelConfirm, confirmCurrent, confirmState } from '../feedback/confirmState';

const open = computed({
  get: () => confirmState.current != null,
  set: (v: boolean) => {
    if (!v) cancelConfirm();
  },
});

const req = computed(() => confirmState.current);

function onConfirm() {
  confirmCurrent();
}

function onCancel() {
  cancelConfirm();
}
</script>

<template>
  <DqDialog
    v-model:open="open"
    :title="req?.title || ''"
    width="420px"
    class="dq-confirm-dialog"
  >
    <p class="dq-confirm-dialog__message">{{ req?.message }}</p>
    <template #footer>
      <DqButton @click="onCancel">{{ req?.options.cancelButtonText || 'Cancel' }}</DqButton>
      <DqButton
        :type="req?.options.type === 'warning' ? 'danger' : 'primary'"
        @click="onConfirm"
      >
        {{ req?.options.confirmButtonText || 'OK' }}
      </DqButton>
    </template>
  </DqDialog>
</template>
