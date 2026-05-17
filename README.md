# DanQing UI (`dq-ui`)

Shared **macOS-native desktop** UI for DanQing product suite (Web + Tauri). **No Element Plus** — Reka UI primitives + Lucide icons.

## Packages

| Package | Description |
|---------|-------------|
| `@danqing/dq-tokens` | Design tokens (`dq-mac.css`, `--dq-*`) |
| `@danqing/dq-ui` | Primitives (`Dq*`) |
| `@danqing/dq-shell` | App chrome: tabs, inspector, pref forms, toast/confirm |

## Quality gate

From this repo (requires sibling `DanQing-Studio`):

```bash
make check
```

Same checks from Studio: `make check-ep-boundary` and `make check-theme-legacy`.

## Rules

- Product apps: `@/utils/feedback` for toasts/confirms; `Dq*` in templates only.
- Only `packages/ui` may import `reka-ui` directly.
- Style changes: tokens + `Dq*`, not third-party component-library overrides.

## Develop with Studio

```bash
cd dq-ui && pnpm install
cd ../DanQing-Studio/frontend && npm install && npm run dev
```
