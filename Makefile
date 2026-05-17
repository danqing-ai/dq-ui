.PHONY: check check-no-ep help

STUDIO_ROOT ?= ../DanQing-Studio
PYTHON ?= python3

help:
	@echo "Targets: check (no Element Plus via Studio scripts)"

check check-no-ep:
	@test -d "$(STUDIO_ROOT)/scripts" || (echo "Set STUDIO_ROOT to DanQing-Studio repo"; exit 1)
	$(PYTHON) "$(STUDIO_ROOT)/scripts/check_ep_boundary.py"
	$(PYTHON) "$(STUDIO_ROOT)/scripts/check_theme_legacy.py"
	$(PYTHON) "$(STUDIO_ROOT)/scripts/check_ui_compat.py"
