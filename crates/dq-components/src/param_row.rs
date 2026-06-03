use crate::control::control_padding_y;
use dq_tokens::{color, spacing, typography};
use iced::widget::{column, container, row, text};
use iced::{Alignment, Element, Length};

/// Compact parameter row: label (80px) | control (fill) on single line.
/// Hint shown as tooltip text below the control, in quaternary color.
pub fn dq_param_row<'a, Message: 'a>(
    label: &'a str,
    control: Element<'a, Message>,
    hint: Option<&'a str>,
) -> Element<'a, Message> {
    let mut body = column![control].spacing(spacing::XS).width(Length::Fill);

    if let Some(h) = hint {
        body = body.push(
            text(h)
                .size(typography::MINI)
                .color(color::text_quaternary()),
        );
    }

    row![
        container(
            text(label)
                .size(typography::CAPTION)
                .color(color::text_tertiary()),
        )
        .width(Length::Fixed(80.0))
        .padding([control_padding_y(), 0.0])
        .align_y(Alignment::Center),
        body,
    ]
    .spacing(spacing::MD)
    .align_y(Alignment::Start)
    .width(Length::Fill)
    .into()
}
