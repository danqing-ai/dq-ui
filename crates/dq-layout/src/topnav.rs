use dq_tokens::{color, layout, spacing, typography};
use dq_theme::topnav_container;
use iced::widget::{container, row, text, Space};
use iced::{Alignment, Element, Length};

pub fn dq_topnav<'a, Message: 'a>(
    title: &'a str,
    subtitle: Option<&'a str>,
    search_hint: &'a str,
    search_shortcut: &'a str,
) -> Element<'a, Message> {
    let title_block: Element<Message> = if let Some(sub) = subtitle {
        row![
            text(title).size(typography::TITLE).color(color::text_primary()),
            text("›")
                .size(typography::BODY)
                .color(color::text_quaternary()),
            text(sub)
                .size(typography::BODY)
                .color(color::text_secondary()),
        ]
        .spacing(spacing::SM)
        .align_y(Alignment::Center)
        .into()
    } else {
        text(title).size(typography::TITLE).color(color::text_primary()).into()
    };

    let search = container(
        row![
            text("⌕")
                .size(typography::BODY)
                .color(color::text_quaternary()),
            text(search_hint)
                .size(typography::LABEL)
                .color(color::text_quaternary()),
            Space::new().width(Length::Fill),
            container(
                text(search_shortcut)
                    .size(typography::CAPTION)
                    .color(color::text_tertiary()),
            )
            .padding([2.0, 5.0])
            .style(|_theme| container::Style {
                background: Some(iced::Background::Color(color::bg_surface())),
                border: iced::Border {
                    color: color::border_subtle(),
                    width: 1.0,
                    radius: spacing::radius_control_sm().into(),
                },
                ..Default::default()
            }),
        ]
        .align_y(Alignment::Center)
        .spacing(spacing::SM)
        .width(Length::Fill),
    )
    .width(Length::Fixed(240.0))
    .height(Length::Fixed(32.0))
    .padding([0.0, spacing::SM])
    .center_y(Length::Fill)
    .style(|_theme| container::Style {
        background: Some(iced::Background::Color(color::bg_inset())),
        border: iced::Border {
            color: color::border_subtle(),
            width: 1.0,
            radius: spacing::radius_control().into(),
        },
        ..Default::default()
    });

    container(
        row![
            title_block,
            Space::new().width(Length::Fill),
            search,
        ]
        .align_y(Alignment::Center)
        .spacing(spacing::MD)
        .width(Length::Fill),
    )
    .height(Length::Fixed(layout::TOPNAV_HEIGHT as f32))
    .padding([0.0, spacing::LG])
    .width(Length::Fill)
    .style(topnav_container)
    .into()
}
