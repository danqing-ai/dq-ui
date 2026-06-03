//! Alert — banner with type-based coloring (info/warning/success/danger).
//! Matches web UI's DqAlert.

use dq_tokens::{color, spacing, typography};
use iced::widget::{container, row, text, Space};
use iced::{Alignment, Element, Length};

/// Alert variant determines the color scheme.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertType {
    Info,
    Success,
    Warning,
    Danger,
}

/// An alert banner — contextual message with optional action button.
pub fn alert<'a, Message: Clone + 'a>(
    message: &'a str,
    alert_type: AlertType,
    action: Option<Element<'a, Message>>,
) -> Element<'a, Message> {
    let (bg, fg, border_color) = match alert_type {
        AlertType::Info => (color::ACCENT_TINT, color::ACCENT, color::ACCENT_MUTED),
        AlertType::Success => (Color::from_rgba(0.24, 0.70, 0.46, 0.10), color::SUCCESS, color::SUCCESS),
        AlertType::Warning => (Color::from_rgba(0.96, 0.76, 0.17, 0.10), color::WARNING, color::WARNING),
        AlertType::Danger => (Color::from_rgba(0.94, 0.27, 0.27, 0.10), color::DANGER, color::DANGER),
    };

    let mut row = row![]
        .spacing(spacing::SM)
        .align_y(Alignment::Center)
        .width(Length::Fill);

    row = row.push(
        text(message)
            .size(typography::LABEL)
            .color(fg)
    );
    row = row.push(Space::new().width(Length::Fill));
    if let Some(a) = action {
        row = row.push(a);
    }

    container(row)
        .padding(spacing::SM)
        .width(Length::Fill)
        .style(move |_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(bg)),
            border: iced::Border {
                color: border_color,
                width: 1.0,
                radius: spacing::RADIUS_MD.into(),
            },
            ..Default::default()
        })
        .into()
}

use iced::Color;
