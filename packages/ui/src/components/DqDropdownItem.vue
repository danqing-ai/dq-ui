<script setup lang="ts">
import { DropdownMenuItem, DropdownMenuSeparator } from 'reka-ui';
import { useDqDropdown } from '../form/dropdownContext';

const props = defineProps<{
  command?: string;
  divided?: boolean;
  disabled?: boolean;
}>();

const dropdown = useDqDropdown();

function onSelect() {
  if (props.disabled || props.command == null) return;
  dropdown?.onCommand(props.command);
}
</script>

<template>
  <DropdownMenuSeparator v-if="divided" class="dq-dropdown-item__separator" />
  <DropdownMenuItem
    class="dq-dropdown-item"
    :disabled="disabled"
    @select="onSelect"
  >
    <slot />
  </DropdownMenuItem>
</template>
