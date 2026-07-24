<script setup lang="ts">
withDefaults(
  defineProps<{
    type?: 'primary' | 'success' | 'warning' | 'danger' | 'info' | 'neutral' | '';
    size?: 'sm' | 'default' | 'large';
    effect?: 'dark' | 'light' | 'plain';
    /** Show a leading status dot (uses type color / running pulse when type is primary) */
    dot?: boolean;
    /** Override dot tone: pending | running | success | warning | danger */
    dotStatus?: 'pending' | 'running' | 'success' | 'warning' | 'danger';
  }>(),
  {
    type: '',
    size: 'default',
    effect: 'light',
    dot: false,
  },
);
</script>

<template>
  <span
    class="dq-tag"
    :class="[
      type ? `dq-tag--${type}` : '',
      size !== 'default' ? `dq-tag--${size}` : '',
      effect ? `dq-tag--${effect}` : '',
      dot ? 'dq-tag--with-dot' : '',
    ]"
  >
    <span
      v-if="dot"
      class="dq-status-dot"
      :class="[
        dotStatus
          ? `dq-status-dot--${dotStatus}`
          : type === 'success'
            ? 'dq-status-dot--success'
            : type === 'warning'
              ? 'dq-status-dot--warning'
              : type === 'danger'
                ? 'dq-status-dot--danger'
                : type === 'primary' || type === 'info'
                  ? 'dq-status-dot--running'
                  : 'dq-status-dot--pending',
      ]"
      aria-hidden="true"
    />
    <slot />
  </span>
</template>
