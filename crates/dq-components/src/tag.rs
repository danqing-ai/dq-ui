//! Tag — colored badge/pill, almost identical to `badge` but with type-based coloring.
//! Matches web UI's DqTag.

use dq_tokens::{color, spacing, typography};
use iced::widget::{container, text};
use iced::{Color, Element};

/// Tag type determines background/border/text color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TagType {
    Default,
    Info,
    Success,
    Warning,
    Danger,
}

/// A small tag/badge — inline label with rounded background.
pub fn tag<'a, Message: Clone + 'a>(
    label: &'a str,
    tag_type: TagType,
) -> Element<'a, Message> {
    let (bg, fg, border) = match tag_type {
        TagType::Success => (color::SUCCESS, Color::WHITE, Color::TRANSPARENT),
        TagType::Danger => (color::DANGER, Color::WHITE, Color::TRANSPARENT),
        TagType::Warning => (color::WARNING, Color::BLACK, Color::TRANSPARENT),
        TagType::Info => (color::ACCENT_TINT, color::ACCENT, Color::TRANSPARENT),
        TagType::Default => (color::BG_SURFACE, color::TEXT_SECONDARY, color::BORDER_SUBTLE),
    };

    container(
        text(label)
            .size(typography::MINI)
            .color(fg),
    )
    .padding([2.0, 6.0])
    .style(move |_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(bg)),
        border: iced::Border {
            color: border,
            width: 1.0,
            radius: spacing::RADIUS_SM.into(),
        },
        ..Default::default()
    })
    .into()
}
