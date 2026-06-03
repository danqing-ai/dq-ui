//! Loading state — spinner + message placeholder.

use dq_tokens::{color, spacing, typography};
use iced::widget::{column, container, text};
use iced::{Alignment, Element, Length};

/// A centered loading state with optional message.
pub fn loading_state<'a, Message: Clone + 'a>(message: &'a str) -> Element<'a, Message> {
    container(
        column![
            crate::phosphor_icon(crate::PhosphorIcon::CircleNotch, 32.0, color::text_tertiary()),
            text(message).size(typography::BODY).color(color::text_secondary()),
        ]
        .spacing(spacing::SM)
        .align_x(Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .into()
}

/// A centered error state with optional retry action.
pub fn error_state<'a, Message: Clone + 'a>(
    message: &'a str,
    retry: Option<Message>,
) -> Element<'a, Message> {
    container(
        column![
            crate::phosphor_icon(crate::PhosphorIcon::WarningCircle, 32.0, color::danger()),
            text("出错了").size(typography::BODY).color(color::text_secondary()),
            text(message).size(typography::CAPTION).color(color::text_tertiary()),
            if let Some(r) = retry {
                crate::dq_button("重试", crate::ButtonVariant::Primary, crate::ButtonSize::Sm, crate::ButtonWidth::Hug, Some(r))
            } else {
                text("").into()
            },
        ]
        .spacing(spacing::SM)
        .align_x(Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .into()
}
