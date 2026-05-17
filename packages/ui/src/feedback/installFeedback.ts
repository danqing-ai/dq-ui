import type { App, Directive } from 'vue';
import { createApp, h } from 'vue';
import DqFeedbackRoot from '../components/DqFeedbackRoot.vue';

type LoadingEl = HTMLElement & { _dqLoadingEl?: HTMLElement };

const vDqLoading: Directive<LoadingEl, boolean> = {
  mounted(el, binding) {
    if (binding.value) attachLoading(el);
  },
  updated(el, binding) {
    if (binding.value) attachLoading(el);
    else detachLoading(el);
  },
  unmounted(el) {
    detachLoading(el);
  },
};

function attachLoading(el: LoadingEl) {
  if (el._dqLoadingEl) return;
  el.style.position = el.style.position || 'relative';
  const overlay = document.createElement('div');
  overlay.className = 'dq-loading-overlay';
  overlay.setAttribute('aria-busy', 'true');
  overlay.innerHTML = '<span class="dq-loading-spinner" aria-hidden="true"></span>';
  el.appendChild(overlay);
  el._dqLoadingEl = overlay;
}

function detachLoading(el: LoadingEl) {
  if (el._dqLoadingEl) {
    el._dqLoadingEl.remove();
    el._dqLoadingEl = undefined;
  }
}

export function installDanQingFeedback(app: App) {
  app.directive('dqLoading', vDqLoading);

  const host = document.createElement('div');
  host.id = 'dq-feedback-root';
  document.body.appendChild(host);
  const feedbackApp = createApp({ render: () => h(DqFeedbackRoot) });
  feedbackApp.mount(host);
}
