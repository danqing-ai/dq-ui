/** Import theme CSS in app entry, e.g. `import '@danqing/dq-tokens/dq-mac.css'` */
export const DQ_TOKENS_VERSION = '0.1.0';

/**
 * Product theme CSS entry files (relative to package root).
 * `glass` / `tauriMacos` are overlays imported after a base palette — not selectable themes.
 */
export const themes = {
  mac: 'dq-mac.css',
  linearDark: 'dq-linear-dark.css',
  chinaRedDark: 'dq-china-red-dark.css',
  shadcnDark: 'dq-shadcn-dark.css',
  shadcnLight: 'dq-shadcn-light.css',
  catppuccin: 'dq-catppuccin.css',
  tokyoNight: 'dq-tokyo-night.css',
  minimalLight: 'dq-minimal-light.css',
  dracula: 'dq-dracula.css',
  nordDark: 'dq-nord-dark.css',
  catppuccinLatte: 'dq-catppuccin-latte.css',
  nordLight: 'dq-nord-light.css',
  githubLight: 'dq-github-light.css',
  /** Overlay utilities — import after a palette theme */
  glass: 'dq-glass.css',
  tauriMacos: 'dq-tauri-macos.css',
} as const;

export type DqThemeId = keyof typeof themes;

/** Kebab-case slug used in localStorage / Settings UI (Teams, Studio). */
export type DqThemeSlug =
  | 'mac'
  | 'linear-dark'
  | 'china-red-dark'
  | 'shadcn-dark'
  | 'shadcn-light'
  | 'catppuccin'
  | 'tokyo-night'
  | 'minimal-light'
  | 'dracula'
  | 'nord-dark'
  | 'catppuccin-latte'
  | 'nord-light'
  | 'github-light';

export interface DqThemeMeta {
  /** CamelCase key matching `themes` / `applyDqTheme` */
  id: Exclude<DqThemeId, 'glass' | 'tauriMacos'>;
  /** Kebab slug for persistence and product Settings */
  slug: DqThemeSlug;
  label: string;
  description: string;
  /** Class added to `<html>` to activate the theme (`mac` uses `dq-mac` for explicit switching) */
  htmlClass: string;
  /** Accent color for preview swatches */
  accent: string;
  /** Whether this is a dark theme (controls the `dark` class on `<html>`) */
  dark: boolean;
  cssFile: string;
}

/** Catalog of selectable product themes (excludes glass / tauri overlays). */
export const THEME_OPTIONS: readonly DqThemeMeta[] = [
  {
    id: 'mac',
    slug: 'mac',
    label: 'macOS',
    description: 'macOS 26 Liquid Glass 原生风格',
    htmlClass: 'dq-mac',
    accent: '#0a84ff',
    dark: true,
    cssFile: themes.mac,
  },
  {
    id: 'linearDark',
    slug: 'linear-dark',
    label: 'Linear Dark',
    description: 'Linear / Figma 风格深色生产力主题',
    htmlClass: 'dq-linear-dark',
    accent: '#6370d2',
    dark: true,
    cssFile: themes.linearDark,
  },
  {
    id: 'chinaRedDark',
    slug: 'china-red-dark',
    label: 'China Red Dark',
    description: '中国红深色主题',
    htmlClass: 'dq-china-red-dark',
    accent: '#C93756',
    dark: true,
    cssFile: themes.chinaRedDark,
  },
  {
    id: 'shadcnDark',
    slug: 'shadcn-dark',
    label: 'shadcn/ui Dark',
    description: 'shadcn/ui 风格 zinc 深色主题',
    htmlClass: 'dq-shadcn-dark',
    accent: '#fafafa',
    dark: true,
    cssFile: themes.shadcnDark,
  },
  {
    id: 'shadcnLight',
    slug: 'shadcn-light',
    label: 'shadcn/ui Light',
    description: 'shadcn/ui 风格暖白亮色主题',
    htmlClass: 'dq-shadcn-light',
    accent: '#18181b',
    dark: false,
    cssFile: themes.shadcnLight,
  },
  {
    id: 'catppuccin',
    slug: 'catppuccin',
    label: 'Catppuccin Mocha',
    description: '暖色柔和暗色主题，护眼舒适',
    htmlClass: 'dq-catppuccin',
    accent: '#cba6f7',
    dark: true,
    cssFile: themes.catppuccin,
  },
  {
    id: 'tokyoNight',
    slug: 'tokyo-night',
    label: 'Tokyo Night',
    description: '霓虹都市暗色主题，高对比度',
    htmlClass: 'dq-tokyo-night',
    accent: '#7aa2f7',
    dark: true,
    cssFile: themes.tokyoNight,
  },
  {
    id: 'minimalLight',
    slug: 'minimal-light',
    label: 'Minimal Light',
    description: '极简纯白亮色主题，专注编码',
    htmlClass: 'dq-minimal-light',
    accent: '#0066cc',
    dark: false,
    cssFile: themes.minimalLight,
  },
  {
    id: 'dracula',
    slug: 'dracula',
    label: 'Dracula',
    description: '经典暗紫开发者主题',
    htmlClass: 'dq-dracula',
    accent: '#bd93f9',
    dark: true,
    cssFile: themes.dracula,
  },
  {
    id: 'nordDark',
    slug: 'nord-dark',
    label: 'Nord Dark',
    description: '北极蓝灰暗色主题，冷静沉稳',
    htmlClass: 'dq-nord-dark',
    accent: '#88c0d0',
    dark: true,
    cssFile: themes.nordDark,
  },
  {
    id: 'catppuccinLatte',
    slug: 'catppuccin-latte',
    label: 'Catppuccin Latte',
    description: '暖色柔和亮色主题，护眼舒适',
    htmlClass: 'dq-catppuccin-latte',
    accent: '#1e66f5',
    dark: false,
    cssFile: themes.catppuccinLatte,
  },
  {
    id: 'nordLight',
    slug: 'nord-light',
    label: 'Nord Light',
    description: '北极冰雪亮色主题，清新明快',
    htmlClass: 'dq-nord-light',
    accent: '#5e81ac',
    dark: false,
    cssFile: themes.nordLight,
  },
  {
    id: 'githubLight',
    slug: 'github-light',
    label: 'GitHub Light',
    description: 'GitHub Primer 亮色主题，开发者首选',
    htmlClass: 'dq-github-light',
    accent: '#0969da',
    dark: false,
    cssFile: themes.githubLight,
  },
] as const;

const SLUG_TO_META = Object.fromEntries(
  THEME_OPTIONS.map((opt) => [opt.slug, opt]),
) as Record<DqThemeSlug, DqThemeMeta>;

const ID_TO_META = Object.fromEntries(
  THEME_OPTIONS.map((opt) => [opt.id, opt]),
) as Record<DqThemeMeta['id'], DqThemeMeta>;

/** Root `<html>` class to activate a theme. Overlays return null. */
export const themeRootClasses: Record<DqThemeId, string | null> = {
  mac: 'dq-mac',
  linearDark: 'dq-linear-dark',
  chinaRedDark: 'dq-china-red-dark',
  shadcnDark: 'dq-shadcn-dark',
  shadcnLight: 'dq-shadcn-light',
  catppuccin: 'dq-catppuccin',
  tokyoNight: 'dq-tokyo-night',
  minimalLight: 'dq-minimal-light',
  dracula: 'dq-dracula',
  nordDark: 'dq-nord-dark',
  catppuccinLatte: 'dq-catppuccin-latte',
  nordLight: 'dq-nord-light',
  githubLight: 'dq-github-light',
  glass: null,
  tauriMacos: 'dq-tauri-macos',
};

/** npm import paths for theme CSS bundles. */
export const themeImportPaths: Record<DqThemeId, string> = {
  mac: '@danqing/dq-tokens/dq-mac.css',
  linearDark: '@danqing/dq-tokens/dq-linear-dark.css',
  chinaRedDark: '@danqing/dq-tokens/dq-china-red-dark.css',
  shadcnDark: '@danqing/dq-tokens/dq-shadcn-dark.css',
  shadcnLight: '@danqing/dq-tokens/dq-shadcn-light.css',
  catppuccin: '@danqing/dq-tokens/dq-catppuccin.css',
  tokyoNight: '@danqing/dq-tokens/dq-tokyo-night.css',
  minimalLight: '@danqing/dq-tokens/dq-minimal-light.css',
  dracula: '@danqing/dq-tokens/dq-dracula.css',
  nordDark: '@danqing/dq-tokens/dq-nord-dark.css',
  catppuccinLatte: '@danqing/dq-tokens/dq-catppuccin-latte.css',
  nordLight: '@danqing/dq-tokens/dq-nord-light.css',
  githubLight: '@danqing/dq-tokens/dq-github-light.css',
  glass: '@danqing/dq-tokens/dq-glass.css',
  tauriMacos: '@danqing/dq-tokens/dq-tauri-macos.css',
};

const ALL_THEME_ROOT_CLASSES = Object.values(themeRootClasses).filter(
  (value): value is string => value != null,
);

function resolveThemeMeta(
  themeIdOrSlug: DqThemeId | DqThemeSlug,
): DqThemeMeta | null {
  if (themeIdOrSlug in ID_TO_META) {
    return ID_TO_META[themeIdOrSlug as DqThemeMeta['id']];
  }
  if (themeIdOrSlug in SLUG_TO_META) {
    return SLUG_TO_META[themeIdOrSlug as DqThemeSlug];
  }
  return null;
}

/**
 * Apply a DanQing theme on `<html>` (browser / Tauri webview).
 * Accepts camelCase ids (`shadcnLight`) or kebab slugs (`shadcn-light`).
 * Toggles `dark` for product themes; overlays (`glass`, `tauriMacos`) only add their class.
 */
export function applyDqTheme(
  themeIdOrSlug: DqThemeId | DqThemeSlug,
  root: HTMLElement = document.documentElement,
): void {
  const meta = resolveThemeMeta(themeIdOrSlug);
  if (meta) {
    root.classList.remove(...ALL_THEME_ROOT_CLASSES);
    root.classList.toggle('dark', meta.dark);
    root.classList.add(meta.htmlClass);
    return;
  }

  // Overlay-only ids
  if (themeIdOrSlug === 'glass') {
    return;
  }
  if (themeIdOrSlug === 'tauriMacos') {
    root.classList.add('dq-tauri-macos');
  }
}

/** Lookup theme metadata by camelCase id or kebab slug. */
export function getDqThemeMeta(
  themeIdOrSlug: DqThemeId | DqThemeSlug,
): DqThemeMeta | null {
  return resolveThemeMeta(themeIdOrSlug);
}

export function isDqThemeSlug(value: string): value is DqThemeSlug {
  return value in SLUG_TO_META;
}
