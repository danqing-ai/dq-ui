//! Semantic color palette — LCH-based, 3-variable generation (base + accent + contrast).
//!
//! Linear derives all surfaces, labels, and borders from **base**, **accent**, and
//! **contrast** (see <https://linear.app/now/how-we-redesigned-the-linear-ui>).
//! Surfaces stay neutral; accent is reserved for interactive chrome.

use iced::Color;

/// Full semantic token set consumed by `dq-theme` and `dq-components`.
#[derive(Debug, Clone, Copy)]
pub struct SemanticPalette {
    // Surfaces — deepest to shallowest (luminance increases)
    pub bg_base: Color,
    pub bg_page: Color,
    pub bg_panel: Color,
    pub bg_elevated: Color,
    pub bg_surface: Color,
    pub bg_inset: Color,
    // Overlays & effects
    pub bg_overlay: Color,
    pub bg_translucent: Color,
    // Accent (interactive only — never mixed into surfaces)
    pub accent: Color,
    pub accent_hover: Color,
    pub accent_muted: Color,
    pub accent_tint: Color,
    // Labels
    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_tertiary: Color,
    pub text_quaternary: Color,
    // Icons (slightly dimmer than labels at same tier)
    pub icon_primary: Color,
    pub icon_secondary: Color,
    pub icon_tertiary: Color,
    // Borders & separators — minimal, used only when necessary
    pub border_subtle: Color,
    pub border_strong: Color,
    pub border_focus: Color,
    pub separator: Color,
    // Fills (hover / selected / active)
    pub fill_hover: Color,
    pub fill_active: Color,
    pub fill_selected: Color,
    // Status
    pub danger: Color,
    pub success: Color,
    pub warning: Color,
}

impl SemanticPalette {
    /// Linear dark theme — dramatic depth layers for hierarchy.
    pub const fn dark() -> Self {
        Self {
            // Deepest layer — sidebar, pure void with cool undertone
            bg_base: Color::from_rgb8(0x07, 0x07, 0x0b),
            // Page background — subtle lift from sidebar
            bg_page: Color::from_rgb8(0x0a, 0x0a, 0x0f),
            // Panel/card surface — workhorse container, barely elevated
            bg_panel: Color::from_rgb8(0x10, 0x10, 0x16),
            // Elevated — hover states, active items
            bg_elevated: Color::from_rgb8(0x18, 0x18, 0x20),
            // Surface — inputs, controls, wells
            bg_surface: Color::from_rgb8(0x20, 0x20, 0x2a),
            // Inset — recessed areas, deeper than page
            bg_inset: Color::from_rgb8(0x08, 0x08, 0x0c),
            bg_overlay: Color::from_rgba(0.0, 0.0, 0.0, 0.70),
            bg_translucent: Color::from_rgba(0.16, 0.16, 0.22, 0.90),
            accent: Color::from_rgb8(0x63, 0x70, 0xd2),
            accent_hover: Color::from_rgb8(0x7d, 0x87, 0xdb),
            accent_muted: Color::from_rgb8(0x55, 0x60, 0xbe),
            accent_tint: Color::from_rgba(0.39, 0.44, 0.82, 0.10),
            text_primary: Color::from_rgba(1.0, 1.0, 1.0, 0.95),
            text_secondary: Color::from_rgba(1.0, 1.0, 1.0, 0.65),
            text_tertiary: Color::from_rgba(1.0, 1.0, 1.0, 0.45),
            text_quaternary: Color::from_rgba(1.0, 1.0, 1.0, 0.28),
            icon_primary: Color::from_rgba(1.0, 1.0, 1.0, 0.85),
            icon_secondary: Color::from_rgba(1.0, 1.0, 1.0, 0.55),
            icon_tertiary: Color::from_rgba(1.0, 1.0, 1.0, 0.38),
            border_subtle: Color::from_rgba(1.0, 1.0, 1.0, 0.04),
            border_strong: Color::from_rgba(1.0, 1.0, 1.0, 0.10),
            border_focus: Color::from_rgb8(0x63, 0x70, 0xd2),
            separator: Color::from_rgba(1.0, 1.0, 1.0, 0.04),
            fill_hover: Color::from_rgba(1.0, 1.0, 1.0, 0.05),
            fill_active: Color::from_rgba(1.0, 1.0, 1.0, 0.08),
            fill_selected: Color::from_rgba(1.0, 1.0, 1.0, 0.11),
            danger: Color::from_rgb8(0xef, 0x44, 0x44),
            success: Color::from_rgb8(0x3e, 0xb3, 0x75),
            warning: Color::from_rgb8(0xf5, 0xc2, 0x2b),
        }
    }

    /// Linear light theme preset.
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
        }
    }
}

/// Default active palette (Linear dark).
pub const LINEAR_DARK: SemanticPalette = SemanticPalette::dark();
