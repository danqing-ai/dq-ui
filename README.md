# DanQing UI (`dq-ui`)

Shared **macOS-native desktop** UI for the DanQing product suite (Web + Tauri). **No Element Plus** — Reka UI primitives + Lucide icons.

## Packages

| Package | Description |
|---------|-------------|
| `@danqing/dq-tokens` | Design tokens (`--dq-*`), product themes, glass / Tauri overlays |
| `@danqing/dq-ui` | Primitives (`Dq*`), icons, toast / confirm feedback |
| `@danqing/dq-shell` | App chrome: inspector, pref forms, command palette, desktop host |

## Quick start

```bash
pnpm install
pnpm run build
pnpm run typecheck
# or
make check
```

`make check` runs workspace `build` + `typecheck` and produces publishable `dist` artifacts.

Clone next to Studio so `file:../../dq-ui/packages/*` resolves:

```bash
git clone https://github.com/danqing-ai/dq-ui.git
cd dq-ui && pnpm install
cd ../DanQing-Studio/frontend && npm install && npm run dev
```

Product-side gates (in Studio): `make check-ep-boundary` and `make check-theme-legacy`.

## Themes

`@danqing/dq-tokens` ships a shared spacing / typography base (`dq-typography.css` → `--dq-space-*`, `--dq-font-size-*`, `--dq-scrollbar-size*`) plus selectable palettes:

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

**Overlays** (import after a base palette; not selectable themes):

| Overlay | CSS import | Notes |
|---------|------------|-------|
| Glass | `dq-glass.css` | Frosted surface utilities |
| Tauri macOS | `dq-tauri-macos.css` | Desktop webview chrome (`dq-tauri-macos` on `<html>`) |

```ts
import '@danqing/dq-tokens/dq-shadcn-light.css';
import '@danqing/dq-tokens/dq-glass.css';
import '@danqing/dq-ui/style.css';
import '@danqing/dq-shell/style.css';
```

Programmatic switching (camelCase id **or** kebab slug; toggles `dark` automatically):

```ts
import { applyDqTheme, THEME_OPTIONS } from '@danqing/dq-tokens';

applyDqTheme('shadcnLight');
applyDqTheme('linear-dark'); // slug also works
// Settings UI: iterate THEME_OPTIONS for label / accent / dark
```

Local previews: open `packages/tokens/demo/index.html` (gallery) or the per-theme demo pages in the same folder.

## `@danqing/dq-ui`

Primitives built on Reka UI + Lucide. Use `Dq*` in templates only; only this package may import `reka-ui` directly.

**Layout / display:** `DqStack`, `DqRow`, `DqCol`, `DqText`, `DqEmpty`, `DqAlert`, `DqTag`, `DqCountBadge`, `DqProgress`, `DqTooltip`, `DqCollapse` / `DqCollapseItem`

**Actions:** `DqButton`, `DqIconButton`, `DqDropdown` / `DqDropdownMenu` / `DqDropdownItem`

**Forms:** `DqInput`, `DqSelect` / `DqOption` (`size="small"` for compact chips), `DqSlider`, `DqSwitch`, `DqCheckbox` / `DqCheckboxGroup`, `DqInputNumber`, `DqDatePicker`

**Navigation:** `DqSegmented`, `DqSectionTabs` / `DqSectionTabTrigger` / `DqSectionTabPanel`

**Overlays:** `DqDialog`, `DqDrawer`

**Icons / feedback:** `DqIcon`, `registerDqIcons`, named Lucide re-exports; `toast`, `confirm`, `installDanQingFeedback`

## `@danqing/dq-shell`

App chrome on top of `@danqing/dq-ui` (re-exports primitives, icons, and feedback):

- **Inspector:** `DqInspectorStack`, `DqInspectorSection`, `DqInspectorList` / `DqInspectorListItem`, `DqInspectorKv`, `DqInspectorCallout`, `DqInspectorEmpty`
- **Preferences:** `DqPrefForm`, `DqPrefPane`, `DqPrefRow`, `DqSurfaceCard`
- **Desktop:** `DqDesktopHost`, `DqCommandPalette`, `useDqDesktopExperience`, `useDqWindowActivity`
- **Commands:** `useDqCommandActions` / `createDqDefaultCommandActions`, `createDqCommandRegistry` / `useDqCommandRegistry`, `useDqRegisterCommands`, `useDqRecentCommands`

Built-in desktop shortcuts via `useDqDesktopExperience`: `mod+k` palette, `mod+,` preferences, `mod+w` close, `mod+1..9` tab switch.

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

## Conventions

- Prefer `--dq-space-xs…xl` over local `--space-*` aliases.
- Use `applyDqTheme` / `THEME_OPTIONS` instead of a private theme class list.
- Focus rings: `--dq-focus-ring`; hover fills: `.dq-hoverable` — do not invent `0 0 0 2px` rings in product CSS.
- Product apps: wrap feedback via `@/utils/feedback`; templates use `Dq*` only.
- Style changes: tokens + `Dq*`, not third-party component-library overrides.

## Repository

https://github.com/danqing-ai/dq-ui
