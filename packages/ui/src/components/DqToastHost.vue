<script setup lang="ts">
import { toastState, dismissToast } from '../feedback/toastState';
</script>

<template>
  <div class="dq-toast-host" aria-live="polite" aria-relevant="additions">
    <TransitionGroup name="dq-toast">
      <article
        v-for="item in toastState.items"
        :key="item.id"
        class="dq-toast"
        :class="`dq-toast--${item.type}`"
        role="status"
      >
        <button
          type="button"
          class="dq-toast__close"
          aria-label="Close"
          @click="dismissToast(item.id)"
        >
          ×
        </button>
        <p v-if="item.title" class="dq-toast__title">{{ item.title }}</p>
        <p class="dq-toast__message">{{ item.message }}</p>
      </article>
    </TransitionGroup>
  </div>
</template>
