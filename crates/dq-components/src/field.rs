use dq_tokens::{color, typography};
use iced::widget::{column, row, text};
use iced::{Alignment, Element, Length};

const FIELD_LABEL_GAP: f32 = 6.0;

pub fn field<'a, Message: 'a>(
    label: &'a str,
    hint: Option<&'a str>,
    content: Element<'a, Message>,
) -> Element<'a, Message> {
    let mut col = column![
        text(label)
            .size(typography::CAPTION)
            .color(color::TEXT_TERTIARY),
        content,
    ]
    .spacing(FIELD_LABEL_GAP)
    .width(Length::Fill)
    .align_x(Alignment::Start);

    if let Some(h) = hint {
        col = col.push(text(h).size(typography::CAPTION).color(color::TEXT_TERTIARY));
    }

    col.into()
}

/// Label on the left, control on the right — for compact rows (CFG slider).
pub fn field_inline<'a, Message: 'a>(
    label: &'a str,
    trailing: Option<Element<'a, Message>>,
    content: Element<'a, Message>,
) -> Element<'a, Message> {
    let label_row = if let Some(trail) = trailing {
        row![
            text(label)
                .size(typography::CAPTION)
                .color(color::TEXT_TERTIARY),
            iced::widget::Space::new().width(Length::Fill),
            trail,
        ]
        .align_y(Alignment::Center)
        .width(Length::Fill)
    } else {
        row![text(label)
            .size(typography::CAPTION)
            .color(color::TEXT_TERTIARY)]
        .width(Length::Fill)
    };

    column![label_row, content]
        .spacing(FIELD_LABEL_GAP)
        .width(Length::Fill)
        .align_x(Alignment::Start)
        .into()
}
