#!/usr/bin/env python3
"""Replace hardcoded rgba/hex in Studio/Teams theme CSS with --dq-* tokens."""
from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]

# Longest keys first to avoid partial replacement.
EXACT_RGBA: dict[str, str] = {
    "rgba(255, 255, 255, 0.94)": "var(--dq-label-on-media)",
    "rgba(255, 255, 255, 0.92)": "var(--dq-label-primary)",
    "rgba(255, 255, 255, 0.88)": "var(--dq-label-on-media)",
    "rgba(255, 255, 255, 0.85)": "var(--dq-label-on-media)",
    "rgba(255, 255, 255, 0.65)": "var(--dq-label-secondary)",
    "rgba(255, 255, 255, 0.22)": "var(--dq-scrollbar-thumb-hover)",
    "rgba(255, 255, 255, 0.18)": "var(--dq-fill-on-glass-hover)",
    "rgba(255, 255, 255, 0.16)": "var(--dq-glass-border-strong)",
    "rgba(255, 255, 255, 0.14)": "var(--dq-fill-on-glass-pressed)",
    "rgba(255, 255, 255, 0.12)": "var(--dq-scrollbar-thumb)",
    "rgba(255, 255, 255, 0.1)": "var(--dq-fill-on-glass)",
    "rgba(255, 255, 255, 0.08)": "var(--dq-fill-on-glass)",
    "rgba(255, 255, 255, 0.06)": "var(--dq-fill-on-glass-subtle)",
    "rgba(255, 255, 255, 0.05)": "var(--dq-surface-inset-hover)",
    "rgba(255, 255, 255, 0.04)": "var(--dq-surface-inset)",
    "rgba(255, 255, 255, 0.03)": "var(--dq-surface-inset)",
    "rgba(235, 235, 245, 0.92)": "var(--dq-label-primary)",
    "rgba(235, 235, 245, 0.85)": "var(--dq-label-secondary)",
    "rgba(235, 235, 245, 0.76)": "var(--dq-label-secondary)",
    "rgba(235, 235, 245, 0.72)": "var(--dq-label-secondary)",
    "rgba(235, 235, 245, 0.6)": "var(--dq-label-tertiary)",
    "rgba(235, 235, 245, 0.56)": "var(--dq-label-desc)",
    "rgba(235, 235, 245, 0.5)": "var(--dq-label-block-title)",
    "rgba(235, 235, 245, 0.45)": "var(--dq-label-footnote)",
    "rgba(235, 235, 245, 0.28)": "var(--dq-label-chevron)",
    "rgba(120, 120, 128, 0.56)": "var(--dq-fill-muted-control-active)",
    "rgba(120, 120, 128, 0.44)": "var(--dq-fill-control-strong)",
    "rgba(120, 120, 128, 0.36)": "var(--dq-fill-muted-control-hover)",
    "rgba(120, 120, 128, 0.32)": "var(--dq-fill-control-hover)",
    "rgba(120, 120, 128, 0.28)": "var(--dq-tag-neutral-bg)",
    "rgba(120, 120, 128, 0.24)": "var(--dq-fill-muted-control)",
    "rgba(120, 120, 128, 0.22)": "var(--dq-fill-muted-surface-hover)",
    "rgba(120, 120, 128, 0.18)": "var(--dq-fill-muted-surface-hover)",
    "rgba(120, 120, 128, 0.16)": "var(--dq-fill-muted-surface-hover)",
    "rgba(120, 120, 128, 0.12)": "var(--dq-fill-muted-surface)",
    "rgba(84, 84, 88, 0.65)": "var(--dq-separator)",
    "rgba(84, 84, 88, 0.55)": "var(--dq-separator-strong)",
    "rgba(84, 84, 88, 0.45)": "var(--dq-separator)",
    "rgba(50, 215, 75, 0.16)": "var(--dq-success-surface-strong)",
    "rgba(50, 215, 75, 0.14)": "var(--dq-success-surface-strong)",
    "rgba(50, 215, 75, 0.1)": "var(--dq-success-surface)",
    "rgba(50, 215, 75, 0.06)": "var(--dq-success-surface)",
    "rgba(50, 215, 75, 0.28)": "var(--dq-success-surface-border)",
    "rgba(50, 215, 75, 0.22)": "var(--dq-success-border)",
    "rgba(10, 132, 255, 0.55)": "var(--dq-accent-glow-strong)",
    "rgba(10, 132, 255, 0.5)": "var(--dq-accent-glow-strong)",
    "rgba(10, 132, 255, 0.45)": "var(--dq-accent-border-hover)",
    "rgba(10, 132, 255, 0.35)": "var(--dq-accent-glow)",
    "rgba(10, 132, 255, 0.34)": "var(--dq-accent-surface-border)",
    "rgba(10, 132, 255, 0.28)": "var(--dq-accent-surface-border)",
    "rgba(10, 132, 255, 0.2)": "var(--dq-accent-ring-subtle)",
    "rgba(10, 132, 255, 0.14)": "var(--dq-surface-list-selected-strong)",
    "rgba(10, 132, 255, 0.1)": "var(--dq-surface-list-selected)",
    "rgba(10, 132, 255, 0.08)": "var(--dq-accent-surface)",
    "rgba(10, 132, 255, 0.04)": "var(--dq-accent-surface)",
    "rgba(255, 159, 10, 0.35)": "var(--dq-warning-border-hover)",
    "rgba(255, 159, 10, 0.22)": "var(--dq-warning-surface-border)",
    "rgba(255, 159, 10, 0.16)": "var(--dq-warning-surface-strong)",
    "rgba(255, 159, 10, 0.14)": "var(--dq-warning-surface-strong)",
    "rgba(255, 159, 10, 0.12)": "var(--dq-warning-surface)",
    "rgba(255, 159, 10, 0.1)": "var(--dq-warning-surface)",
    "rgba(255, 159, 10, 0.08)": "var(--dq-warning-surface)",
    "rgba(255, 69, 58, 0.45)": "var(--dq-danger-icon-muted)",
    "rgba(255, 69, 58, 0.42)": "var(--dq-danger-icon-muted)",
    "rgba(255, 69, 58, 0.35)": "var(--dq-danger-border-hover)",
    "rgba(255, 69, 58, 0.28)": "var(--dq-danger-surface-border)",
    "rgba(255, 69, 58, 0.22)": "var(--dq-danger-surface-border)",
    "rgba(255, 69, 58, 0.14)": "var(--dq-danger-surface-strong)",
    "rgba(255, 69, 58, 0.12)": "var(--dq-danger-surface)",
    "rgba(255, 69, 58, 0.1)": "var(--dq-danger-surface)",
    "rgba(255, 69, 58, 0.08)": "var(--dq-danger-surface)",
    "rgba(255, 59, 58, 0.45)": "var(--dq-danger-icon-muted)",
    "rgba(255, 59, 58, 0.1)": "var(--dq-danger-surface)",
    "rgba(0, 0, 0, 0.88)": "var(--dq-overlay-scrim)",
    "rgba(0, 0, 0, 0.78)": "var(--dq-overlay-gradient-end)",
    "rgba(0, 0, 0, 0.65)": "var(--dq-overlay-heavy)",
    "rgba(0, 0, 0, 0.62)": "var(--dq-overlay-card)",
    "rgba(0, 0, 0, 0.55)": "var(--dq-overlay-card)",
    "rgba(0, 0, 0, 0.52)": "var(--dq-overlay-card)",
    "rgba(0, 0, 0, 0.45)": "var(--dq-shadow-popover)",
    "rgba(0, 0, 0, 0.42)": "var(--dq-shadow-lg)",
    "rgba(0, 0, 0, 0.4)": "var(--dq-shadow-lg)",
    "rgba(0, 0, 0, 0.35)": "var(--dq-overlay-medium)",
    "rgba(0, 0, 0, 0.32)": "var(--dq-shadow-md)",
    "rgba(0, 0, 0, 0.28)": "var(--dq-mask-light)",
    "rgba(0, 0, 0, 0.26)": "var(--dq-shadow-md)",
    "rgba(0, 0, 0, 0.25)": "var(--dq-overlay-light)",
    "rgba(0, 0, 0, 0.24)": "var(--dq-shadow-sm)",
    "rgba(0, 0, 0, 0.12)": "var(--dq-shadow-sm)",
    "rgba(60, 60, 67, 0.3)": "var(--dq-label-footnote)",
    "rgba(15, 23, 42, 0.12)": "var(--dq-accent-ring-subtle)",
    "rgba(15, 23, 42, 0.1)": "var(--dq-accent-ring-subtle)",
    "rgba(255, 255, 255, 0.95)": "var(--dq-label-emphasis)",
    "rgba(255, 255, 255, 0.45)": "var(--dq-label-footnote)",
    "rgba(255, 255, 255, 0.28)": "var(--dq-fill-muted-control)",
    "rgba(255, 255, 255, 0.2)": "var(--dq-fill-subtle)",
    "rgba(255, 255, 255, 0.18)": "var(--dq-scrollbar-thumb-hover)",
    "rgba(255, 255, 255, 0.11)": "var(--dq-fill-faint-hover)",
    "rgba(255, 255, 255, 0.07)": "var(--dq-fill-faint)",
    "rgba(28, 28, 30, 0.85)": "var(--dq-lightbox-info-bg)",
    "rgba(0, 0, 0, 0.72)": "var(--dq-overlay-deep)",
    "rgba(10, 132, 255, 0.05)": "var(--dq-accent-gradient-start)",
    "rgba(50, 215, 75, 0.18)": "var(--dq-success-highlight)",
    "rgba(255, 159, 10, 0.18)": "var(--dq-warning-highlight)",
    "rgba(50, 215, 75, 0.12)": "var(--dq-success-surface)",
    "rgba(50, 215, 75, 0.1)": "var(--dq-success-surface)",
    "rgba(120, 120, 128, 0.2)": "var(--dq-fill-muted-row)",
    "rgba(120, 120, 128, 0.16)": "var(--dq-fill-muted-surface-hover)",
    "rgba(120, 120, 128, 0.1)": "var(--dq-fill-muted-surface-medium)",
    "rgba(120, 120, 128, 0.08)": "var(--dq-fill-muted-surface-light)",
    "rgba(235, 235, 245, 0.55)": "var(--dq-label-muted-strong)",
    "rgba(48, 209, 88, 0.12)": "var(--dq-success-surface)",
    "rgba(48, 209, 88, 0.28)": "var(--dq-success-surface-border)",
    "rgba(255, 59, 48, 0.1)": "var(--dq-danger-surface)",
    "rgba(255, 59, 48, 0.45)": "var(--dq-danger-icon-muted)",
}

EXACT_HEX: dict[str, str] = {
    "#ffffff": "var(--dq-color-white)",
    "#fff": "var(--dq-color-white)",
    "#0a84ff": "var(--dq-accent)",
    "#0077ed": "var(--dq-accent-hover)",
    "#1c1c1e": "var(--dq-bg-base)",
    "#2c2c2e": "var(--dq-bg-elevated)",
    "#000000": "var(--dq-bg-page)",
}

THEME_FILES = [
    "theme-apple-dark.css",
    "theme-apple-native.css",
    "theme-apple-finish.css",
    "theme.css",
]

PRODUCTS = ["DanQing-Studio", "DanQing-Teams"]


def tokenize_text(text: str) -> str:
    for old, new in sorted(EXACT_RGBA.items(), key=lambda x: -len(x[0])):
        text = text.replace(old, new)
    for old, new in sorted(EXACT_HEX.items(), key=lambda x: -len(x[0])):
        # Do not replace inside var() definitions in dq-mac — only theme files
        text = re.sub(rf"(?<!-)\B{re.escape(old)}\b", new, text)
    return text


def main() -> int:
    total_left = 0
    for product in PRODUCTS:
        styles = ROOT / product / "frontend" / "src" / "styles"
        if not styles.is_dir():
            continue
        for name in THEME_FILES:
            path = styles / name
            if not path.exists():
                continue
            original = path.read_text(encoding="utf-8")
            updated = tokenize_text(original)
            if updated != original:
                path.write_text(updated, encoding="utf-8")
            left = len(re.findall(r"rgba\([^)]+\)", updated))
            hex_left = len(
                re.findall(
                    r"(?<!var\(--dq-)(?<!#)[#][0-9a-fA-F]{3,8}\b",
                    updated,
                )
            )
            total_left += left
            print(f"{product}/{name}: rgba={left}")
    print(f"total rgba remaining (approx): {total_left}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
