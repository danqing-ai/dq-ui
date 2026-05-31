use dq_tokens::{color, spacing, typography};
use iced::widget::{container, text};
use iced::{Element, Length};

/// Toast notification — top-right or inline, auto-dismissing.
/// Static data version (no timer). Caller manages lifecycle.
pub fn dq_toast<'a, Message: Clone + 'a>(
    message: &'a str,
    variant: ToastVariant,
) -> Element<'a, Message> {
    let tint = match variant {
        ToastVariant::Success => color::SUCCESS,
        ToastVariant::Error => color::DANGER,
        ToastVariant::Warning => color::WARNING,
        ToastVariant::Info => color::ACCENT,
    };

    container(
        text(message)
            .size(typography::CAPTION)
            .color(color::TEXT_PRIMARY),
    )
    .padding([spacing::SM, spacing::MD])
    .width(Length::Shrink)
    .style(move |_theme| iced::widget::container::Style {
        background: Some(iced::Background::Color(color::BG_ELEVATED)),
        border: iced::Border {
            color: tint,
            width: 1.0,
            radius: spacing::RADIUS_MD.into(),
        },
        shadow: iced::Shadow {
            color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.30),
            offset: iced::Vector::new(0.0, 2.0),
            blur_radius: 8.0,
        },
        ..Default::default()
    })
    .into()
}

#[derive(Debug, Clone, Copy)]
pub enum ToastVariant {
    Success,
    Error,
    Warning,
    Info,
}
