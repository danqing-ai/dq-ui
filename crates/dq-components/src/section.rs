use crate::icons::{section_icon, SectionIcon};
use dq_tokens::{color, spacing, typography};
use iced::widget::{column, container, row, text, Space};
use iced::{Alignment, Element, Length};

/// Linear-style panel: single surface, hairline separator, compact.
/// No separate header background — depth comes from luminance, not shadow.
pub fn dq_section<'a, Message: Clone + 'a>(
    icon: SectionIcon,
    title: &'a str,
    trailing: Option<Element<'a, Message>>,
    body: Option<Element<'a, Message>>,
) -> Element<'a, Message> {
    // Header row — single fixed-height row, everything center-aligned
    let header_row = row![
        section_icon(icon),
        text(title)
            .size(typography::LABEL)
            .color(color::text_secondary()),
    ]
    .spacing(6.0)
    .align_y(Alignment::Center)
    .height(Length::Fixed(20.0))
    .width(Length::Fill);

    let mut header = header_row;
    header = header.push(Space::new().width(Length::Fill));

    if let Some(action) = trailing {
        header = header.push(action);
    }

    let header_container = container(header)
        .padding([spacing::XS, spacing::MD])
        .width(Length::Fill);

    // Build panel content
    let panel_content = if let Some(content) = body {
        column![
            header_container,
            container(Space::new())
                .width(Length::Fill)
                .height(Length::Fixed(1.0))
                .style(|_theme| container::Style {
                    background: Some(iced::Background::Color(color::border_subtle())),
                    ..Default::default()
                }),
            container(content)
                .padding([spacing::SM, spacing::MD])
                .width(Length::Fill),
        ]
        .spacing(0.0)
        .width(Length::Fill)
    } else {
        column![header_container]
            .spacing(0.0)
            .width(Length::Fill)
    };

    // Single surface shell — depth from luminance, not shadow
    container(panel_content)
        .width(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(color::bg_panel())),
            border: iced::Border {
                color: color::border_subtle(),
                width: 1.0,
                radius: spacing::radius_control().into(),
            },
            ..Default::default()
        })
        .into()
}
