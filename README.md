# DanQing UI (`dq-ui`)

Shared **macOS-native desktop** UI for DanQing product suite (Web + Tauri). **No Element Plus** — Reka UI primitives + Lucide icons.

## Packages

| Package | Description |
|---------|-------------|
| `@danqing/dq-tokens` | Design tokens (`dq-mac.css`, `--dq-*`) and productivity themes |
| `@danqing/dq-ui` | Primitives (`Dq*`) |
| `@danqing/dq-shell` | App chrome: tabs, inspector, pref forms, toast/confirm |

## Quality gate

From this repo (requires sibling `DanQing-Studio`):

```bash
make check
```

Same checks from Studio: `make check-ep-boundary` and `make check-theme-legacy`.

Library packages can now be built to publishable `dist` artifacts:

```bash
npm run build
npm run typecheck
```

## Themes

`@danqing/dq-tokens` ships a shared spacing/typography base (`dq-typography.css` → `--dq-space-*`, `--dq-font-size-*`, `--dq-scrollbar-size*`) plus selectable product palettes:

| Theme | Slug | CSS import | `<html>` class | Dark |
|-------|------|------------|----------------|------|
| macOS | `mac` | `dq-mac.css` | `dq-mac` | yes |
| Linear Dark | `linear-dark` | `dq-linear-dark.css` | `dq-linear-dark` | yes |
| China Red Dark | `china-red-dark` | `dq-china-red-dark.css` | `dq-china-red-dark` | yes |
| shadcn/ui Dark | `shadcn-dark` | `dq-shadcn-dark.css` | `dq-shadcn-dark` | yes |
| shadcn/ui Light | `shadcn-light` | `dq-shadcn-light.css` | `dq-shadcn-light` | no |
| Catppuccin Mocha | `catppuccin` | `dq-catppuccin.css` | `dq-catppuccin` | yes |
| Tokyo Night | `tokyo-night` | `dq-tokyo-night.css` | `dq-tokyo-night` | yes |
| Minimal Light | `minimal-light` | `dq-minimal-light.css` | `dq-minimal-light` | no |
| Dracula | `dracula` | `dq-dracula.css` | `dq-dracula` | yes |
| Nord Dark | `nord-dark` | `dq-nord-dark.css` | `dq-nord-dark` | yes |
| Catppuccin Latte | `catppuccin-latte` | `dq-catppuccin-latte.css` | `dq-catppuccin-latte` | no |
| Nord Light | `nord-light` | `dq-nord-light.css` | `dq-nord-light` | no |
| GitHub Light | `github-light` | `dq-github-light.css` | `dq-github-light` | no |

Import glass surface utilities after the base theme:

```ts
import '@danqing/dq-tokens/dq-shadcn-light.css';
import '@danqing/dq-tokens/dq-glass.css';
import '@danqing/dq-ui/style.css';
```

Programmatic theme switching (camelCase id **or** kebab slug; toggles `dark` automatically):

```ts
import { applyDqTheme, THEME_OPTIONS } from '@danqing/dq-tokens';

applyDqTheme('shadcnLight');
applyDqTheme('linear-dark'); // slug also works
// Settings UI: iterate THEME_OPTIONS for label / accent / dark
```

**Studio / product migration notes**

- Prefer `--dq-space-xs…xl` over local `--space-*` aliases.
- Use `applyDqTheme` / `THEME_OPTIONS` instead of maintaining a private theme class list.
- Focus rings: `--dq-focus-ring`; hover fills: `.dq-hoverable` — do not invent `0 0 0 2px` rings in product CSS.
- Management tabs: `DqSectionTabs` / `DqSegmented`; selects: `DqSelect` (`size="small"` for compact chips).

Local previews: open `packages/tokens/demo/index.html` (gallery) or the per-theme demo pages in the same folder.

- Product apps: `@/utils/feedback` for toasts/confirms; `Dq*` in templates only.
- Only `packages/ui` may import `reka-ui` directly.
- Style changes: tokens + `Dq*`, not third-party component-library overrides.

## Desktop interaction utilities

`@danqing/dq-shell` now includes:

- `DqDesktopHost`: one-stop desktop behavior host (command palette + shortcuts + optional window active/inactive tracking).
- `DqCommandPalette`: command palette dialog with keyboard navigation.
- `useDqDesktopExperience`: desktop shortcut manager (`mod+k` toggles palette, `mod+,` opens preferences callback, `mod+w` close current, `mod+1..9` tab switch callbacks).
- `useDqCommandActions` / `createDqDefaultCommandActions`: merge app commands with built-in desktop commands (preferences, reload, back/forward).
- `useDqRecentCommands`: store recent command history for ranking.
- `createDqCommandRegistry` / `useDqCommandRegistry`: dynamically register commands from feature modules at runtime.
- `useDqRegisterCommands`: register/unregister module commands automatically with component lifecycle.
- `useDqWindowActivity`: toggles `is-active`/`is-inactive` on `<html>` from window focus state (for macOS inactive-window styling).

Example:

```vue
<script setup lang="ts">
import { computed } from 'vue';
import {
  DqDesktopHost,
  useDqRegisterCommands,
  type DqCommandAction,
} from '@danqing/dq-shell';

const featureCommands = computed<DqCommandAction[]>(() => [
  {
    id: 'feature.new-item',
    title: 'Create Item',
    shortcut: 'mod+n',
    run: () => {
      // feature action
    },
  },
]);

useDqRegisterCommands(featureCommands);
</script>

<template>
  <DqDesktopHost />
</template>
```

## Repository

https://github.com/danqing-ai/dq-ui

Clone next to Studio (sibling directory) so `file:../../dq-ui/packages/*` resolves:

```bash
git clone https://github.com/danqing-ai/dq-ui.git
```

## Develop with Studio

```bash
cd dq-ui && pnpm install
cd ../DanQing-Studio/frontend && npm install && npm run dev
```
