//! Semantic tokens — values sourced from `packages/tokens/src/dq-*-dark.css` (web UI baseline).

use iced::Color;
use std::sync::{OnceLock, RwLock};

/// Layout metrics mirrored from web `--dq-radius-*` / `--dq-row-gutter`.
#[derive(Debug, Clone, Copy)]
pub struct ThemeMetrics {
    pub radius_group: f32,
    pub radius_control: f32,
    pub radius_control_sm: f32,
    pub radius_button: f32,
    pub radius_input: f32,
    pub row_gutter: f32,
}

/// Full semantic token set consumed by `dq-theme` and `dq-components`.
#[derive(Debug, Clone, Copy)]
pub struct SemanticPalette {
    pub bg_base: Color,
    pub bg_page: Color,
    pub bg_panel: Color,
    pub bg_elevated: Color,
    pub bg_surface: Color,
    pub bg_inset: Color,
    pub bg_overlay: Color,
    pub bg_translucent: Color,
    pub accent: Color,
    pub accent_hover: Color,
    pub accent_muted: Color,
    pub accent_tint: Color,
    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_tertiary: Color,
    pub text_quaternary: Color,
    pub icon_primary: Color,
    pub icon_secondary: Color,
    pub icon_tertiary: Color,
    pub border_subtle: Color,
    pub border_strong: Color,
    pub border_focus: Color,
    pub separator: Color,
    pub fill_hover: Color,
    pub fill_active: Color,
    pub fill_selected: Color,
    pub danger: Color,
    pub success: Color,
    pub warning: Color,
    pub success_surface: Color,
    pub warning_surface: Color,
    pub danger_surface: Color,
    pub scrollbar_thumb: Color,
    pub scrollbar_thumb_hover: Color,
}

#[derive(Debug, Clone, Copy)]
struct ActiveTheme {
    palette: SemanticPalette,
    metrics: ThemeMetrics,
}

static ACTIVE: OnceLock<RwLock<ActiveTheme>> = OnceLock::new();

fn active_store() -> &'static RwLock<ActiveTheme> {
    ACTIVE.get_or_init(|| {
        RwLock::new(ActiveTheme {
            palette: SemanticPalette::linear_dark(),
            metrics: ThemeMetrics::linear_dark(),
        })
    })
}

/// Install palette + layout metrics (call from `dq_theme::theme_from_palette`).
pub fn set_active_theme(palette: SemanticPalette, metrics: ThemeMetrics) {
    *active_store().write().expect("theme lock poisoned") = ActiveTheme { palette, metrics };
}

pub fn active_palette() -> SemanticPalette {
    active_store().read().expect("theme lock poisoned").palette
}

pub fn active_metrics() -> ThemeMetrics {
    active_store().read().expect("theme lock poisoned").metrics
}

impl ThemeMetrics {
    /// `dq-linear-dark.css`
    pub const fn linear_dark() -> Self {
        Self {
            radius_group: 10.0,
            radius_control: 8.0,
            radius_control_sm: 6.0,
            radius_button: 6.0,
            radius_input: 6.0,
            row_gutter: 16.0,
        }
    }

    /// `dq-china-red-dark.css`
    pub const fn china_red_dark() -> Self {
        Self {
            radius_group: 8.0,
            radius_control: 6.0,
            radius_control_sm: 4.0,
            radius_button: 4.0,
            radius_input: 4.0,
            row_gutter: 16.0,
        }
    }
}

impl SemanticPalette {
    /// Linear dark — `packages/tokens/src/dq-linear-dark.css`
    pub const fn linear_dark() -> Self {
        Self {
            // --dq-material-chrome / --dq-bg-base
            bg_base: Color::from_rgb8(0x07, 0x07, 0x0b),
            // --dq-bg-page
            bg_page: Color::from_rgb8(0x0a, 0x0a, 0x0f),
            // --dq-bg-elevated
            bg_panel: Color::from_rgb8(0x10, 0x10, 0x16),
            // --dq-surface-elevated
            bg_elevated: Color::from_rgb8(0x18, 0x18, 0x20),
            // --dq-fill-control
            bg_surface: Color::from_rgb8(0x20, 0x20, 0x2a),
            // --dq-fill-dim
            bg_inset: Color::from_rgb8(0x08, 0x08, 0x0c),
            // --dq-glass-scrim
            bg_overlay: Color::from_rgba(0.0, 0.0, 0.0, 0.70),
            // --dq-glass-bar-bg
            bg_translucent: Color::from_rgba(0.027, 0.027, 0.043, 0.85),
            // --dq-accent
            accent: Color::from_rgb8(0x63, 0x70, 0xd2),
            accent_hover: Color::from_rgb8(0x7d, 0x87, 0xdb),
            accent_muted: Color::from_rgb8(0x55, 0x60, 0xbe),
            accent_tint: Color::from_rgba(0.388, 0.439, 0.824, 0.10),
            // --dq-label-*
            text_primary: Color::from_rgba(1.0, 1.0, 1.0, 0.95),
            text_secondary: Color::from_rgba(1.0, 1.0, 1.0, 0.65),
            text_tertiary: Color::from_rgba(1.0, 1.0, 1.0, 0.45),
            text_quaternary: Color::from_rgba(1.0, 1.0, 1.0, 0.28),
            icon_primary: Color::from_rgba(1.0, 1.0, 1.0, 0.85),
            icon_secondary: Color::from_rgba(1.0, 1.0, 1.0, 0.55),
            icon_tertiary: Color::from_rgba(1.0, 1.0, 1.0, 0.38),
            // --dq-border-* / --dq-separator
            border_subtle: Color::from_rgba(1.0, 1.0, 1.0, 0.10),
            border_strong: Color::from_rgba(1.0, 1.0, 1.0, 0.18),
            border_focus: Color::from_rgb8(0x63, 0x70, 0xd2),
            separator: Color::from_rgba(1.0, 1.0, 1.0, 0.08),
            // --dq-fill-on-glass-hover / pressed / control-selected
            fill_hover: Color::from_rgba(1.0, 1.0, 1.0, 0.09),
            fill_active: Color::from_rgba(1.0, 1.0, 1.0, 0.11),
            fill_selected: Color::from_rgba(0.388, 0.439, 0.824, 0.14),
            danger: Color::from_rgb8(0xef, 0x44, 0x44),
            success: Color::from_rgb8(0x3e, 0xb3, 0x75),
            warning: Color::from_rgb8(0xf5, 0xc2, 0x2b),
            success_surface: Color::from_rgba(0.243, 0.702, 0.459, 0.08),
            warning_surface: Color::from_rgba(0.961, 0.761, 0.169, 0.10),
            danger_surface: Color::from_rgba(0.937, 0.267, 0.267, 0.10),
            scrollbar_thumb: Color::from_rgba(1.0, 1.0, 1.0, 0.08),
            scrollbar_thumb_hover: Color::from_rgba(1.0, 1.0, 1.0, 0.15),
        }
    }

    /// Back-compat alias.
    pub const fn dark() -> Self {
        Self::linear_dark()
    }

    /// Linear light theme preset (web light theme not shipped yet).
    pub const fn light() -> Self {
        Self {
            bg_base: Color::from_rgb8(0xfc, 0xfc, 0xfc),
            bg_page: Color::from_rgb8(0xf5, 0xf5, 0xf7),
            bg_panel: Color::from_rgb8(0xff, 0xff, 0xff),
            bg_elevated: Color::from_rgb8(0xff, 0xff, 0xff),
            bg_surface: Color::from_rgb8(0xff, 0xff, 0xff),
            bg_inset: Color::from_rgb8(0xf8, 0xf8, 0xfa),
            bg_overlay: Color::from_rgba(0.0, 0.0, 0.0, 0.35),
            bg_translucent: Color::from_rgba(1.0, 1.0, 1.0, 0.90),
            accent: Color::from_rgb8(0x5e, 0x6a, 0xd2),
            accent_hover: Color::from_rgb8(0x4f, 0x57, 0xc4),
            accent_muted: Color::from_rgb8(0x6e, 0x79, 0xdb),
            accent_tint: Color::from_rgba(0.37, 0.42, 0.82, 0.08),
            text_primary: Color::from_rgba(0.0, 0.0, 0.0, 0.90),
            text_secondary: Color::from_rgba(0.0, 0.0, 0.0, 0.62),
            text_tertiary: Color::from_rgba(0.0, 0.0, 0.0, 0.42),
            text_quaternary: Color::from_rgba(0.0, 0.0, 0.0, 0.28),
            icon_primary: Color::from_rgba(0.0, 0.0, 0.0, 0.82),
            icon_secondary: Color::from_rgba(0.0, 0.0, 0.0, 0.55),
            icon_tertiary: Color::from_rgba(0.0, 0.0, 0.0, 0.38),
            border_subtle: Color::from_rgba(0.0, 0.0, 0.0, 0.06),
            border_strong: Color::from_rgba(0.0, 0.0, 0.0, 0.12),
            border_focus: Color::from_rgb8(0x5e, 0x6a, 0xd2),
            separator: Color::from_rgba(0.0, 0.0, 0.0, 0.05),
            fill_hover: Color::from_rgba(0.0, 0.0, 0.0, 0.04),
            fill_active: Color::from_rgba(0.0, 0.0, 0.0, 0.06),
            fill_selected: Color::from_rgba(0.0, 0.0, 0.0, 0.08),
            danger: Color::from_rgb8(0xd9, 0x3c, 0x3c),
            success: Color::from_rgb8(0x2e, 0xa0, 0x5a),
            warning: Color::from_rgb8(0xc9, 0x8a, 0x1a),
            success_surface: Color::from_rgba(0.18, 0.63, 0.35, 0.08),
            warning_surface: Color::from_rgba(0.79, 0.54, 0.04, 0.10),
            danger_surface: Color::from_rgba(0.85, 0.24, 0.24, 0.10),
            scrollbar_thumb: Color::from_rgba(0.0, 0.0, 0.0, 0.12),
            scrollbar_thumb_hover: Color::from_rgba(0.0, 0.0, 0.0, 0.22),
        }
    }

    /// China Red dark — `packages/tokens/src/dq-china-red-dark.css`
    pub const fn china_red_dark() -> Self {
        Self {
            // --dq-material-chrome / --dq-bg-base (sidebar chrome)
            bg_base: Color::from_rgb8(0x14, 0x10, 0x10),
            // --dq-bg-page (page void)
            bg_page: Color::from_rgb8(0x0c, 0x0a, 0x0a),
            // --dq-bg-elevated
            bg_panel: Color::from_rgb8(0x1c, 0x16, 0x16),
            // --dq-surface-elevated
            bg_elevated: Color::from_rgb8(0x24, 0x1c, 0x1c),
            // --dq-fill-control
            bg_surface: Color::from_rgb8(0x2c, 0x22, 0x22),
            // --dq-fill-dim
            bg_inset: Color::from_rgb8(0x10, 0x0c, 0x0c),
            // --dq-glass-scrim
            bg_overlay: Color::from_rgba(0.047, 0.039, 0.039, 0.65),
            // --dq-glass-bar-bg
            bg_translucent: Color::from_rgba(0.078, 0.063, 0.063, 0.88),
            accent: Color::from_rgb8(0xc9, 0x37, 0x56),
            accent_hover: Color::from_rgb8(0xd8, 0x4a, 0x68),
            accent_muted: Color::from_rgb8(0xe0, 0x60, 0x7e),
            accent_tint: Color::from_rgba(0.788, 0.216, 0.337, 0.12),
            text_primary: Color::from_rgba(1.0, 0.980, 0.961, 0.92),
            text_secondary: Color::from_rgba(1.0, 0.961, 0.933, 0.62),
            text_tertiary: Color::from_rgba(1.0, 0.941, 0.910, 0.40),
            text_quaternary: Color::from_rgba(1.0, 0.922, 0.886, 0.26),
            icon_primary: Color::from_rgba(1.0, 0.980, 0.961, 0.82),
            icon_secondary: Color::from_rgba(1.0, 0.961, 0.933, 0.55),
            icon_tertiary: Color::from_rgba(1.0, 0.941, 0.910, 0.38),
            border_subtle: Color::from_rgba(0.788, 0.216, 0.337, 0.05),
            border_strong: Color::from_rgba(0.788, 0.216, 0.337, 0.14),
            border_focus: Color::from_rgb8(0xc9, 0x37, 0x56),
            separator: Color::from_rgba(0.788, 0.216, 0.337, 0.10),
            fill_hover: Color::from_rgba(0.788, 0.216, 0.337, 0.08),
            fill_active: Color::from_rgba(0.788, 0.216, 0.337, 0.12),
            fill_selected: Color::from_rgba(0.788, 0.216, 0.337, 0.18),
            danger: Color::from_rgb8(0xb4, 0x3c, 0x3c),
            success: Color::from_rgb8(0x5b, 0x8c, 0x5a),
            warning: Color::from_rgb8(0xd4, 0xa5, 0x74),
            success_surface: Color::from_rgba(0.357, 0.549, 0.353, 0.10),
            warning_surface: Color::from_rgba(0.831, 0.647, 0.455, 0.12),
            danger_surface: Color::from_rgba(0.706, 0.235, 0.235, 0.12),
            scrollbar_thumb: Color::from_rgba(0.788, 0.216, 0.337, 0.12),
            scrollbar_thumb_hover: Color::from_rgba(0.788, 0.216, 0.337, 0.22),
        }
    }
}

/// Default preset (Linear dark).
pub const LINEAR_DARK: SemanticPalette = SemanticPalette::linear_dark();
