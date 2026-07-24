<script setup lang="ts">
import { computed, useSlots } from 'vue'

export interface DqPillTabItem {
  value: string
  label: string
  badge?: string | number
  icon?: unknown
}

const props = withDefaults(
  defineProps<{
    items: DqPillTabItem[]
    size?: 'default' | 'sm'
  }>(),
  { size: 'sm' },
)

const model = defineModel<string>({ required: true })
const slots = useSlots()

const rootClass = computed(() => [
  'dq-pill-tabs',
  props.size === 'sm' ? 'dq-pill-tabs--sm' : '',
])
</script>

<template>
  <div :class="rootClass" role="tablist">
    <button
      v-for="item in items"
      :key="item.value"
      type="button"
      role="tab"
      class="dq-pill-tabs__tab"
      :class="{ 'is-active': model === item.value }"
      :aria-selected="model === item.value"
      @click="model = item.value"
    >
      <component :is="item.icon" v-if="item.icon" class="dq-pill-tabs__icon" :size="14" />
      <slot v-if="slots[`icon-${item.value}`]" :name="`icon-${item.value}`" />
      <span class="dq-pill-tabs__label">{{ item.label }}</span>
      <span v-if="item.badge != null && item.badge !== ''" class="dq-pill-tabs__badge">{{ item.badge }}</span>
    </button>
  </div>
</template>
