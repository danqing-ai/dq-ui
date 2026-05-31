//! Runtime theme generation — Linear-style base + accent + contrast.
//!
//! Surfaces are derived from **neutral** base luminance steps (no accent tint).
//! Label opacities scale with `contrast` for accessibility (Linear contrast 30–100).

use crate::semantic::SemanticPalette;
use iced::Color;

/// Theme inputs — three knobs like Linear's custom theme generator.
#[derive(Debug, Clone, Copy)]
pub struct ThemeInput {
    /// Application chrome base (sidebar / deepest layer).
    pub base: Color,
    /// Brand / interactive accent — not mixed into surfaces.
    pub accent: Color,
    /// 0.0 (soft) … 1.0 (high contrast, a11y).
    pub contrast: f32,
    pub is_dark: bool,
}

impl ThemeInput {
    pub const fn linear_dark_default() -> Self {
        Self {
            base: Color::from_rgb8(0x07, 0x07, 0x08),
            accent: Color::from_rgb8(0x5e, 0x6a, 0xd2),
            contrast: 0.40,
            is_dark: true,
        }
    }
}

/// Generate a full semantic palette from theme inputs.
pub fn generate(input: ThemeInput) -> SemanticPalette {
    if input.is_dark {
        generate_dark(input)
    } else {
        generate_light(input)
    }
}

fn generate_dark(input: ThemeInput) -> SemanticPalette {
    let c = input.contrast.clamp(0.0, 1.0);
    let base = input.base;
    let accent = input.accent;

    // Surfaces: deeper layers get less elevation, creating depth through luminance
    let bg_page = elevate(base, 2);
    let bg_panel = elevate(base, 6);
    let bg_elevated = elevate(base, 12);
    let bg_surface = elevate(base, 20);
    let bg_inset = depress(base, 1);

    SemanticPalette {
        bg_base: base,
        bg_page,
        bg_panel,
        bg_elevated,
        bg_surface,
        bg_inset,
        bg_overlay: Color::from_rgba(0.0, 0.0, 0.0, 0.60),
        bg_translucent: Color::from_rgba(
            bg_elevated.r,
            bg_elevated.g,
            bg_elevated.b,
            0.85,
        ),
        accent,
        accent_hover: lighten(accent, 0.10),
        accent_muted: darken(accent, 0.08),
        accent_tint: Color::from_rgba(accent.r, accent.g, accent.b, 0.10),
        text_primary: Color::from_rgba(1.0, 1.0, 1.0, 0.90 + c * 0.06),
        text_secondary: Color::from_rgba(1.0, 1.0, 1.0, 0.55 + c * 0.12),
        text_tertiary: Color::from_rgba(1.0, 1.0, 1.0, 0.38 + c * 0.10),
        text_quaternary: Color::from_rgba(1.0, 1.0, 1.0, 0.24 + c * 0.08),
        icon_primary: Color::from_rgba(1.0, 1.0, 1.0, 0.82 + c * 0.08),
        icon_secondary: Color::from_rgba(1.0, 1.0, 1.0, 0.52 + c * 0.10),
        icon_tertiary: Color::from_rgba(1.0, 1.0, 1.0, 0.36 + c * 0.08),
        border_subtle: Color::from_rgba(1.0, 1.0, 1.0, 0.05 + c * 0.02),
        border_strong: Color::from_rgba(1.0, 1.0, 1.0, 0.10 + c * 0.04),
        border_focus: accent,
        separator: Color::from_rgba(1.0, 1.0, 1.0, 0.04 + c * 0.02),
        fill_hover: Color::from_rgba(1.0, 1.0, 1.0, 0.04 + c * 0.02),
        fill_active: Color::from_rgba(1.0, 1.0, 1.0, 0.06 + c * 0.03),
        fill_selected: Color::from_rgba(1.0, 1.0, 1.0, 0.08 + c * 0.04),
        danger: Color::from_rgb8(0xeb, 0x57, 0x57),
        success: Color::from_rgb8(0x4c, 0xb7, 0x82),
        warning: Color::from_rgb8(0xf2, 0xc9, 0x4c),
    }
}

fn generate_light(input: ThemeInput) -> SemanticPalette {
    let c = input.contrast.clamp(0.0, 1.0);
    let base = input.base;
    let accent = input.accent;

    let bg_page = depress(base, 2);
    let bg_panel = Color::from_rgb8(0xff, 0xff, 0xff);
    let bg_elevated = Color::from_rgb8(0xff, 0xff, 0xff);
    let bg_surface = Color::from_rgb8(0xff, 0xff, 0xff);
    let bg_inset = depress(base, 1);

    SemanticPalette {
        bg_base: base,
        bg_page,
        bg_panel,
        bg_elevated,
        bg_surface,
        bg_inset,
        bg_overlay: Color::from_rgba(0.0, 0.0, 0.0, 0.35),
        bg_translucent: Color::from_rgba(1.0, 1.0, 1.0, 0.90),
        accent,
        accent_hover: darken(accent, 0.08),
        accent_muted: lighten(accent, 0.10),
        accent_tint: Color::from_rgba(accent.r, accent.g, accent.b, 0.08),
        text_primary: Color::from_rgba(0.0, 0.0, 0.0, 0.85 + c * 0.07),
        text_secondary: Color::from_rgba(0.0, 0.0, 0.0, 0.52 + c * 0.12),
        text_tertiary: Color::from_rgba(0.0, 0.0, 0.0, 0.36 + c * 0.10),
        text_quaternary: Color::from_rgba(0.0, 0.0, 0.0, 0.22 + c * 0.08),
        icon_primary: Color::from_rgba(0.0, 0.0, 0.0, 0.78 + c * 0.08),
        icon_secondary: Color::from_rgba(0.0, 0.0, 0.0, 0.50 + c * 0.10),
        icon_tertiary: Color::from_rgba(0.0, 0.0, 0.0, 0.34 + c * 0.08),
        border_subtle: Color::from_rgba(0.0, 0.0, 0.0, 0.05 + c * 0.02),
        border_strong: Color::from_rgba(0.0, 0.0, 0.0, 0.10 + c * 0.04),
        border_focus: accent,
        separator: Color::from_rgba(0.0, 0.0, 0.0, 0.04 + c * 0.02),
        fill_hover: Color::from_rgba(0.0, 0.0, 0.0, 0.03 + c * 0.02),
        fill_active: Color::from_rgba(0.0, 0.0, 0.0, 0.05 + c * 0.03),
        fill_selected: Color::from_rgba(0.0, 0.0, 0.0, 0.07 + c * 0.04),
        danger: Color::from_rgb8(0xd9, 0x3c, 0x3c),
        success: Color::from_rgb8(0x2e, 0xa0, 0x5a),
        warning: Color::from_rgb8(0xc9, 0x8a, 0x1a),
    }
}

fn elevate(color: Color, step: u8) -> Color {
    rgb_step(color, step as i16)
}

fn depress(color: Color, step: u8) -> Color {
    rgb_step(color, -(step as i16))
}

fn rgb_step(color: Color, step: i16) -> Color {
    let [r, g, b, _] = color.into_rgba8();
    Color::from_rgb8(
        (r as i16 + step).clamp(0, 255) as u8,
        (g as i16 + step).clamp(0, 255) as u8,
        (b as i16 + step).clamp(0, 255) as u8,
    )
}

fn lighten(color: Color, amount: f32) -> Color {
    mix(color, Color::WHITE, amount)
}

fn darken(color: Color, amount: f32) -> Color {
    mix(color, Color::BLACK, amount)
}

fn mix(a: Color, b: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    Color::from_rgb(
        a.r + (b.r - a.r) * t,
        a.g + (b.g - a.g) * t,
        a.b + (b.b - a.b) * t,
    )
}
