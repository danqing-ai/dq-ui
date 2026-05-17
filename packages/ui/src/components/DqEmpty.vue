<script setup lang="ts">
defineProps<{
  description?: string;
}>();
</script>

<template>
  <div class="dq-empty" role="status">
    <div v-if="$slots.image" class="dq-empty__image">
      <slot name="image" />
    </div>
    <p v-if="description" class="dq-empty__description">{{ description }}</p>
    <p v-else-if="$slots.default" class="dq-empty__description">
      <slot />
    </p>
    <div v-if="$slots.extra" class="dq-empty__extra">
      <slot name="extra" />
    </div>
  </div>
</template>
