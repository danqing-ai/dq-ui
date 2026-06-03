//! Card — `DqSurfaceCard` equivalent: elevated panel with optional header and trailing.
//! Matches web UI's DqSurfaceCard pattern.

use dq_tokens::{color, spacing, typography};
use iced::widget::{column, container, row, text, Space};
use iced::{Alignment, Element, Length};

/// A surface card — elevated panel with header slot + optional trailing action + body.
pub fn surface_card<'a, Message: Clone + 'a>(
    header: Option<Element<'a, Message>>,
    trailing: Option<Element<'a, Message>>,
    body: Option<Element<'a, Message>>,
) -> Element<'a, Message> {
    let mut head = row![].spacing(spacing::SM).align_y(Alignment::Center).width(Length::Fill);

    if let Some(h) = header {
        head = head.push(h);
    }
    head = head.push(Space::new().width(Length::Fill));
    if let Some(t) = trailing {
        head = head.push(t);
    }

    let mut content = column![].spacing(spacing::MD).width(Length::Fill);
    content = content.push(head);
    if let Some(b) = body {
        content = content.push(b);
    }

    container(content)
        .padding(spacing::MD)
        .width(Length::Fill)
        .style(|_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(color::bg_panel())),
            border: iced::Border {
                color: color::border_subtle(),
                width: 1.0,
                radius: spacing::radius_group().into(),
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

/// Simple card with just a title and body.
pub fn section_card<'a, Message: Clone + 'a>(
    title: &'a str,
    body: Element<'a, Message>,
) -> Element<'a, Message> {
    surface_card(
        Some(text(title).size(typography::TITLE).color(color::text_primary()).into()),
        None,
        Some(body),
    )
}
