#!/usr/bin/env python3
"""List theme-apple-* selectors whose root class never appears in Studio frontend src."""
from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
STUDIO_SRC = ROOT / "DanQing-Studio" / "frontend" / "src"
THEME_FILES = [
    STUDIO_SRC / "styles" / "theme-apple-dark.css",
    STUDIO_SRC / "styles" / "theme-apple-native.css",
    STUDIO_SRC / "styles" / "theme-apple-finish.css",
]

SKIP_ROOTS = {
    "html",
    "body",
    "dark",
    "root",
    "from",
    "to",
    "keyframes",
    "media",
}


def collect_sources() -> str:
    parts: list[str] = []
    for ext in ("*.vue", "*.ts", "*.html"):
        for path in STUDIO_SRC.rglob(ext):
            if "node_modules" in path.parts:
                continue
            try:
                parts.append(path.read_text(encoding="utf-8", errors="ignore"))
            except OSError:
                pass
    return "\n".join(parts)


def root_classes(selector: str) -> set[str]:
    selector = selector.split(",")[0].strip()
    selector = re.sub(r"::[a-z-]+", "", selector)
    selector = re.sub(r":[a-z-]+(?:\([^)]*\))?", "", selector)
    parts = selector.split()
    if not parts:
        return set()
    root = parts[0].lstrip(".")
    if not root or root.startswith("#") or root in SKIP_ROOTS:
        return set()
    return {root}


def extract_rules(css: str) -> list[tuple[str, str]]:
    rules: list[tuple[str, str]] = []
    for block in re.finditer(r"([^{}]+)\{([^{}]*)\}", css, re.S):
        sel, body = block.group(1).strip(), block.group(2).strip()
        if not sel or sel.startswith("@") or "keyframes" in sel:
            continue
        for part in sel.split(","):
            part = part.strip()
            if part:
                rules.append((part, body))
    return rules


def main() -> int:
    haystack = collect_sources()
    orphans: list[str] = []
    for path in THEME_FILES:
        if not path.exists():
            continue
        css = path.read_text(encoding="utf-8")
        for sel, body in extract_rules(css):
            roots = root_classes(sel)
            if not roots:
                continue
            if any(
                re.search(rf"(class\s*=\s*['\"][^'\"]*\b{re.escape(r)}\b)|\.{re.escape(r)}\b", haystack)
                for r in roots
            ):
                continue
            if not body.strip():
                orphans.append(f"{path.name}: {sel} {{}}")
            elif len(body) < 120:
                orphans.append(f"{path.name}: {sel}")

    if not orphans:
        print("No obvious orphan selectors (heuristic).")
        return 0
    print(f"Possible orphan selectors ({len(orphans)}):")
    for line in orphans[:80]:
        print(f"  - {line}")
    if len(orphans) > 80:
        print(f"  … and {len(orphans) - 80} more")
    return 0


if __name__ == "__main__":
    sys.exit(main())
