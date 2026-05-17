<script setup lang="ts">
defineProps<{
  title?: string;
  last?: boolean;
}>();
</script>

<template>
  <section
    class="dq-inspector-section"
    :class="{ 'dq-inspector-section--last': last }"
  >
    <header
      v-if="title || $slots.actions"
      class="dq-inspector-section__head"
    >
      <h3
        v-if="title"
        class="dq-inspector-section__title"
      >
        {{ title }}
      </h3>
      <div
        v-if="$slots.actions"
        class="dq-inspector-section__actions"
      >
        <slot name="actions" />
      </div>
    </header>
    <slot />
  </section>
</template>
