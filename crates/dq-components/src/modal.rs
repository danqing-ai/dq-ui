use dq_tokens::{color, spacing, typography};
use iced::widget::{column, container, row, text, Space};
use iced::{Alignment, Element, Length};

/// Modal overlay — full-screen backdrop + centered panel.
pub fn dq_modal<'a, Message: Clone + 'a>(
    title: &'a str,
    body: Element<'a, Message>,
    footer: Element<'a, Message>,
) -> Element<'a, Message> {
    let panel = container(
        column![
            row![
                text(title)
                    .size(typography::TITLE)
                    .color(color::TEXT_PRIMARY),
                Space::new().width(Length::Fill),
            ]
            .align_y(Alignment::Center)
            .width(Length::Fill),
            container(body)
                .padding([spacing::MD, 0.0])
                .width(Length::Fill),
            footer,
        ]
        .spacing(spacing::MD)
        .width(Length::Fill),
    )
    .padding(spacing::XL)
    .width(Length::Fixed(440.0))
    .style(|_theme| iced::widget::container::Style {
        background: Some(iced::Background::Color(color::BG_ELEVATED)),
        border: iced::Border {
            color: color::BORDER_SUBTLE,
            width: 1.0,
            radius: spacing::RADIUS_LG.into(),
        },
        shadow: iced::Shadow {
            color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.50),
            offset: iced::Vector::new(0.0, 4.0),
            blur_radius: 16.0,
        },
        ..Default::default()
    });

    container(panel)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(|_theme| iced::widget::container::Style {
            background: Some(iced::Background::Color(color::BG_OVERLAY)),
            ..Default::default()
        })
        .into()
}
