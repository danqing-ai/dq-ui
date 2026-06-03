//! Key-Value display row — read-only label + value pair.
//! Matches web UI's DqInspectorKv pattern.

use dq_tokens::{color, spacing, typography};
use iced::widget::{row, text, Space};
use iced::{Alignment, Element, Length};

/// A key-value display row.
pub fn kv_row<'a, Message: Clone + 'a>(
    key: &'a str,
    value: &'a str,
) -> Element<'a, Message> {
    row![
        text(key).size(typography::LABEL).color(color::TEXT_SECONDARY),
        Space::new().width(Length::Fill),
        text(value).size(typography::BODY).color(color::TEXT_PRIMARY),
    ]
    .spacing(spacing::SM)
    .align_y(Alignment::Center)
    .width(Length::Fill)
    .into()
}

/// A key-value display with a progress bar underneath.
pub fn kv_row_with_bar<'a, Message: Clone + 'a>(
    key: &'a str,
    value: &'a str,
    progress: f32,
) -> Element<'a, Message> {
    let bar = crate::dq_progress_bar(progress, 4.0);
    column![
        kv_row(key, value),
        bar,
    ]
    .spacing(4.0)
    .width(Length::Fill)
    .into()
}

use iced::widget::column;
