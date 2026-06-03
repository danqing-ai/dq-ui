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
        TagType::Success => (color::success(), Color::WHITE, Color::TRANSPARENT),
        TagType::Danger => (color::danger(), Color::WHITE, Color::TRANSPARENT),
        TagType::Warning => (color::warning(), Color::BLACK, Color::TRANSPARENT),
        TagType::Info => (color::accent_tint(), color::accent(), Color::TRANSPARENT),
        TagType::Default => (color::bg_surface(), color::text_secondary(), color::border_subtle()),
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
            radius: spacing::radius_control_sm().into(),
        },
        ..Default::default()
    })
    .into()
}
