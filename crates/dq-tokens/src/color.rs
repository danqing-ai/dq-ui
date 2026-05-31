//! Flat color aliases — backward-compatible re-exports from the active palette.
//!
//! Prefer `semantic::LINEAR_DARK` or `generate()` for new code.

use crate::semantic::LINEAR_DARK;
use iced::Color;

// Surfaces
pub const BG_PAGE: Color = LINEAR_DARK.bg_page;
pub const BG_BASE: Color = LINEAR_DARK.bg_base;
pub const BG_ELEVATED: Color = LINEAR_DARK.bg_elevated;
pub const BG_PANEL: Color = LINEAR_DARK.bg_panel;
pub const BG_SURFACE: Color = LINEAR_DARK.bg_surface;
pub const BG_INSET: Color = LINEAR_DARK.bg_inset;
pub const BG_OVERLAY: Color = LINEAR_DARK.bg_overlay;
pub const BG_TRANSLUCENT: Color = LINEAR_DARK.bg_translucent;

// Accent
pub const ACCENT: Color = LINEAR_DARK.accent;
pub const ACCENT_HOVER: Color = LINEAR_DARK.accent_hover;
pub const ACCENT_MUTED: Color = LINEAR_DARK.accent_muted;
pub const ACCENT_TINT: Color = LINEAR_DARK.accent_tint;

// Labels
pub const TEXT_PRIMARY: Color = LINEAR_DARK.text_primary;
pub const TEXT_SECONDARY: Color = LINEAR_DARK.text_secondary;
pub const TEXT_TERTIARY: Color = LINEAR_DARK.text_tertiary;
pub const TEXT_QUATERNARY: Color = LINEAR_DARK.text_quaternary;

// Icons
pub const ICON_PRIMARY: Color = LINEAR_DARK.icon_primary;
pub const ICON_SECONDARY: Color = LINEAR_DARK.icon_secondary;
pub const ICON_TERTIARY: Color = LINEAR_DARK.icon_tertiary;

// Borders
pub const BORDER_SUBTLE: Color = LINEAR_DARK.border_subtle;
pub const BORDER_STRONG: Color = LINEAR_DARK.border_strong;
pub const BORDER_FOCUS: Color = LINEAR_DARK.border_focus;
pub const SEPARATOR: Color = LINEAR_DARK.separator;

// Fills
pub const FILL_HOVER: Color = LINEAR_DARK.fill_hover;
pub const FILL_ACTIVE: Color = LINEAR_DARK.fill_active;
pub const FILL_SELECTED: Color = LINEAR_DARK.fill_selected;
pub const FILL_NAV_ACTIVE: Color = LINEAR_DARK.fill_selected;

// Status
pub const DANGER: Color = LINEAR_DARK.danger;
pub const SUCCESS: Color = LINEAR_DARK.success;
pub const WARNING: Color = LINEAR_DARK.warning;
