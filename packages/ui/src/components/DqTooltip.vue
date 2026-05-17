<script setup lang="ts">
import {
  TooltipContent,
  TooltipPortal,
  TooltipProvider,
  TooltipRoot,
  TooltipTrigger,
} from 'reka-ui';

withDefaults(
  defineProps<{
    content?: string;
    placement?: 'top' | 'bottom' | 'left' | 'right';
    showAfter?: number;
    disabled?: boolean;
  }>(),
  {
    placement: 'top',
    showAfter: 400,
    disabled: false,
  },
);
</script>

<template>
  <TooltipProvider :delay-duration="showAfter" :disabled="disabled">
    <TooltipRoot>
      <TooltipTrigger as-child>
        <slot />
      </TooltipTrigger>
      <TooltipPortal>
        <TooltipContent :side="placement" class="dq-tooltip-content">
          <slot name="content">{{ content }}</slot>
        </TooltipContent>
      </TooltipPortal>
    </TooltipRoot>
  </TooltipProvider>
</template>
