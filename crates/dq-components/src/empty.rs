//! Empty state placeholder — matches web UI DqEmpty pattern.

use dq_tokens::{color, spacing, typography};
use iced::widget::{column, container, text};
use iced::{Alignment, Element, Length};

/// An empty state with icon, title, description, and optional action.
pub fn empty_state<'a, Message: Clone + 'a>(
    icon: Option<crate::PhosphorIcon>,
    title: &'a str,
    description: &'a str,
    action: Option<Element<'a, Message>>,
) -> Element<'a, Message> {
    let mut col = column![].spacing(spacing::MD).align_x(Alignment::Center);

    if let Some(ic) = icon {
        col = col.push(crate::phosphor_icon(ic, 40.0, color::text_tertiary()));
    }

    col = col.push(text(title).size(typography::BODY).color(color::text_secondary()));
    col = col.push(text(description).size(typography::CAPTION).color(color::text_tertiary()));

    if let Some(a) = action {
        col = col.push(a);
    }

    container(col)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .padding(spacing::XXL)
        .into()
}
