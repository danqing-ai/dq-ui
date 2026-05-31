use dq_tokens::color;
use iced::widget::{column, container, row, Space};
use iced::{Alignment, Element, Length};

use crate::phosphor::{phosphor_icon, phosphor_icon_box, PhosphorIcon};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StudioIcon {
    Image,
    Video,
    Audio,
    Gallery,
    Models,
    Settings,
    Queue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionIcon {
    Cube,
    Document,
    Pencil,
    Gear,
    Clock,
    Sliders,
}

fn stroke(active: bool) -> iced::Color {
    if active {
        color::ICON_PRIMARY
    } else {
        color::ICON_SECONDARY
    }
}

fn section_stroke() -> iced::Color {
    color::ICON_SECONDARY
}

/// Fixed-size box — never use `center_x(Fill)` or the node expands to row width.
fn icon_box<'a, Message: 'a>(
    content: Element<'a, Message>,
    size: f32,
) -> Element<'a, Message> {
    container(content)
        .width(Length::Fixed(size))
        .height(Length::Fixed(size))
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .into()
}

/// Brand logo — clean sparkle icon, no border/background.
pub fn brand_logo<'a, Message: 'a>() -> Element<'a, Message> {
    use crate::phosphor::phosphor_icon;
    phosphor_icon(crate::phosphor::PhosphorIcon::Sparkle, 22.0, color::ACCENT)
}

pub fn list_icon<'a, Message: 'a>() -> Element<'a, Message> {
    phosphor_icon(PhosphorIcon::List, 14.0, color::ICON_SECONDARY)
}

pub fn sparkle_icon<'a, Message: 'a>(tint: iced::Color) -> Element<'a, Message> {
    phosphor_icon(PhosphorIcon::Sparkle, 14.0, tint)
}

pub fn section_icon<'a, Message: 'a>(icon: SectionIcon) -> Element<'a, Message> {
    let tint = section_stroke();
    let phosphor = match icon {
        SectionIcon::Cube => PhosphorIcon::Cube,
        SectionIcon::Document => PhosphorIcon::FileText,
        SectionIcon::Pencil => PhosphorIcon::Pencil,
        SectionIcon::Gear => PhosphorIcon::Gear,
        SectionIcon::Clock => PhosphorIcon::Clock,
        SectionIcon::Sliders => PhosphorIcon::Sliders,
    };
    // Return raw icon (no icon_box) so the parent row can align it precisely.
    phosphor_icon(phosphor, 14.0, tint)
}

pub fn image_placeholder_icon<'a, Message: 'a>() -> Element<'a, Message> {
    phosphor_icon_box(PhosphorIcon::Image, 20.0, color::TEXT_QUATERNARY, 24.0)
}

/// 20×20 Phosphor icons for sidebar navigation.
pub fn studio_icon<'a, Message: 'a>(icon: StudioIcon, active: bool) -> Element<'a, Message> {
    let tint = stroke(active);
    let phosphor = match icon {
        StudioIcon::Image => PhosphorIcon::Image,
        StudioIcon::Video => PhosphorIcon::VideoCamera,
        StudioIcon::Audio => PhosphorIcon::SpeakerHigh,
        StudioIcon::Gallery => PhosphorIcon::Images,
        StudioIcon::Models => PhosphorIcon::Cube,
        StudioIcon::Settings => PhosphorIcon::Gear,
        StudioIcon::Queue => PhosphorIcon::List,
    };
    icon_box(phosphor_icon(phosphor, 20.0, tint), 20.0)
}

/// Chevron arrow for expand/collapse
pub fn chevron_icon<'a, Message: 'a>(down: bool) -> Element<'a, Message> {
    let icon = if down { PhosphorIcon::CaretDown } else { PhosphorIcon::CaretRight };
    phosphor_icon(icon, 10.0, color::ICON_SECONDARY)
}

/// Refresh / reload
pub fn refresh_icon<'a, Message: 'a>() -> Element<'a, Message> {
    phosphor_icon(PhosphorIcon::ArrowsClockwise, 14.0, color::ICON_TERTIARY)
}

/// Small accent thumbnail placeholder.
pub fn thumb_icon<'a, Message: 'a>() -> Element<'a, Message> {
    phosphor_icon_box(PhosphorIcon::Image, 16.0, color::ACCENT, 16.0)
}
