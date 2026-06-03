/** Import theme CSS in app entry, e.g. `import '@danqing/dq-tokens/dq-mac.css'` */
export const DQ_TOKENS_VERSION = '0.1.0';

/** Published theme CSS entry files (relative to package root). */
export const themes = {
  mac: 'dq-mac.css',
  glass: 'dq-glass.css',
  tauriMacos: 'dq-tauri-macos.css',
  linearDark: 'dq-linear-dark.css',
  chinaRedDark: 'dq-china-red-dark.css',
} as const;

export type DqThemeId = keyof typeof themes;

/** Root `<html>` class to activate a theme (mac uses `:root` tokens — no class required). */
export const themeRootClasses: Record<DqThemeId, string | null> = {
  mac: null,
  glass: null,
  tauriMacos: 'dq-tauri-macos',
  linearDark: 'dq-linear-dark',
  chinaRedDark: 'dq-china-red-dark',
};

/** npm import paths for theme CSS bundles. */
export const themeImportPaths: Record<DqThemeId, string> = {
  mac: '@danqing/dq-tokens/dq-mac.css',
  glass: '@danqing/dq-tokens/dq-glass.css',
  tauriMacos: '@danqing/dq-tokens/dq-tauri-macos.css',
  linearDark: '@danqing/dq-tokens/dq-linear-dark.css',
  chinaRedDark: '@danqing/dq-tokens/dq-china-red-dark.css',
};

const ALL_THEME_ROOT_CLASSES = Object.values(themeRootClasses).filter(
  (value): value is string => value != null,
);

/** Apply a DanQing theme class on `<html>` (browser / Tauri webview). */
export function applyDqTheme(
  themeId: DqThemeId,
  root: HTMLElement = document.documentElement,
): void {
  root.classList.remove(...ALL_THEME_ROOT_CLASSES);
  const themeClass = themeRootClasses[themeId];
  if (themeClass) {
    root.classList.add(themeClass);
  }
}
