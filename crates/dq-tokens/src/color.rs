//! Runtime color accessors — always read the active theme palette.

use crate::semantic::active_palette;
use iced::Color;

#[inline]
pub fn bg_page() -> Color {
    active_palette().bg_page
}
#[inline]
pub fn bg_base() -> Color {
    active_palette().bg_base
}
#[inline]
pub fn bg_elevated() -> Color {
    active_palette().bg_elevated
}
#[inline]
pub fn bg_panel() -> Color {
    active_palette().bg_panel
}
#[inline]
pub fn bg_surface() -> Color {
    active_palette().bg_surface
}
#[inline]
pub fn bg_inset() -> Color {
    active_palette().bg_inset
}
#[inline]
pub fn bg_overlay() -> Color {
    active_palette().bg_overlay
}
#[inline]
pub fn bg_translucent() -> Color {
    active_palette().bg_translucent
}

#[inline]
pub fn accent() -> Color {
    active_palette().accent
}
#[inline]
pub fn accent_hover() -> Color {
    active_palette().accent_hover
}
#[inline]
pub fn accent_muted() -> Color {
    active_palette().accent_muted
}
#[inline]
pub fn accent_tint() -> Color {
    active_palette().accent_tint
}

#[inline]
pub fn text_primary() -> Color {
    active_palette().text_primary
}
#[inline]
pub fn text_secondary() -> Color {
    active_palette().text_secondary
}
#[inline]
pub fn text_tertiary() -> Color {
    active_palette().text_tertiary
}
#[inline]
pub fn text_quaternary() -> Color {
    active_palette().text_quaternary
}

#[inline]
pub fn icon_primary() -> Color {
    active_palette().icon_primary
}
#[inline]
pub fn icon_secondary() -> Color {
    active_palette().icon_secondary
}
#[inline]
pub fn icon_tertiary() -> Color {
    active_palette().icon_tertiary
}

#[inline]
pub fn border_subtle() -> Color {
    active_palette().border_subtle
}
#[inline]
pub fn border_strong() -> Color {
    active_palette().border_strong
}
#[inline]
pub fn border_focus() -> Color {
    active_palette().border_focus
}
#[inline]
pub fn separator() -> Color {
    active_palette().separator
}

#[inline]
pub fn fill_hover() -> Color {
    active_palette().fill_hover
}
#[inline]
pub fn fill_active() -> Color {
    active_palette().fill_active
}
#[inline]
pub fn fill_selected() -> Color {
    active_palette().fill_selected
}
#[inline]
pub fn fill_nav_active() -> Color {
    active_palette().fill_selected
}

#[inline]
pub fn danger() -> Color {
    active_palette().danger
}
#[inline]
pub fn success() -> Color {
    active_palette().success
}
#[inline]
pub fn warning() -> Color {
    active_palette().warning
}
#[inline]
pub fn success_surface() -> Color {
    active_palette().success_surface
}
#[inline]
pub fn warning_surface() -> Color {
    active_palette().warning_surface
}
#[inline]
pub fn danger_surface() -> Color {
    active_palette().danger_surface
}
#[inline]
pub fn scrollbar_thumb() -> Color {
    active_palette().scrollbar_thumb
}
#[inline]
pub fn scrollbar_thumb_hover() -> Color {
    active_palette().scrollbar_thumb_hover
}
