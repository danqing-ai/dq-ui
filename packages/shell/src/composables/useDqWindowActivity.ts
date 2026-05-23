import { onBeforeUnmount, onMounted, toValue, type MaybeRefOrGetter } from 'vue';

export interface UseDqWindowActivityOptions {
  enabled?: MaybeRefOrGetter<boolean>;
  target?: HTMLElement;
  inactiveClassName?: string;
  activeClassName?: string;
}

export function useDqWindowActivity(options: UseDqWindowActivityOptions = {}) {
  const target = options.target ?? document.documentElement;
  const inactiveClassName = options.inactiveClassName ?? 'is-inactive';
  const activeClassName = options.activeClassName ?? 'is-active';

  const applyActive = () => {
    if (toValue(options.enabled) === false) return;
    target.classList.add(activeClassName);
    target.classList.remove(inactiveClassName);
  };

  const applyInactive = () => {
    if (toValue(options.enabled) === false) return;
    target.classList.remove(activeClassName);
    target.classList.add(inactiveClassName);
  };

  const onVisibilityChange = () => {
    if (document.hidden) {
      applyInactive();
      return;
    }
    applyActive();
  };

  onMounted(() => {
    applyActive();
    window.addEventListener('focus', applyActive);
    window.addEventListener('blur', applyInactive);
    document.addEventListener('visibilitychange', onVisibilityChange);
  });

  onBeforeUnmount(() => {
    window.removeEventListener('focus', applyActive);
    window.removeEventListener('blur', applyInactive);
    document.removeEventListener('visibilitychange', onVisibilityChange);
  });

  return {
    setActive: applyActive,
    setInactive: applyInactive,
  };
}
