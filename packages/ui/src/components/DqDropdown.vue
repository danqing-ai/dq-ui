<script setup lang="ts">
import {
  DropdownMenuContent,
  DropdownMenuPortal,
  DropdownMenuRoot,
  DropdownMenuTrigger,
} from 'reka-ui';
import { provideDqDropdown } from '../form/dropdownContext';

withDefaults(
  defineProps<{
    /** Menu open trigger; only `click` is implemented (default). */
    trigger?: 'click' | 'hover' | 'contextmenu';
  }>(),
  { trigger: 'click' },
);

const emit = defineEmits<{
  command: [command: string];
}>();

provideDqDropdown((command) => emit('command', command));
</script>

<template>
  <DropdownMenuRoot class="dq-dropdown">
    <DropdownMenuTrigger as-child>
      <slot />
    </DropdownMenuTrigger>
    <DropdownMenuPortal>
      <DropdownMenuContent class="dq-dropdown__content" align="end" :side-offset="6">
        <slot name="dropdown" />
      </DropdownMenuContent>
    </DropdownMenuPortal>
  </DropdownMenuRoot>
</template>
