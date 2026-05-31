use dq_tokens::{color, spacing, typography};
use iced::widget::{column, container, row, text, Space};
use iced::{Element, Length};

/// List item for Inbox / Mail — Linear-style row with hover, optional accent left border.
pub fn dq_list_item<'a, Message: Clone + 'a>(
    title: &'a str,
    subtitle: Option<&'a str>,
    meta: Option<&'a str>,
    active: bool,
    unread: bool,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let title_color = if unread {
        color::TEXT_PRIMARY
    } else {
        color::TEXT_SECONDARY
    };

    let mut content = column![
        text(title)
            .size(typography::BODY)
            .color(title_color),
    ]
    .spacing(2.0)
    .width(Length::Fill);

    if let Some(sub) = subtitle {
        content = content.push(
            text(sub)
                .size(typography::CAPTION)
                .color(color::TEXT_TERTIARY),
        );
    }

    let mut row_content = row![content].width(Length::Fill);

    if let Some(m) = meta {
        row_content = row_content.push(
            text(m)
                .size(typography::MINI)
                .color(color::TEXT_QUATERNARY),
        );
    }

    let indicator = if active {
        Some(
            container(Space::new())
                .width(Length::Fixed(2.0))
                .height(Length::Fill)
                .style(|_theme| iced::widget::container::Style {
                    background: Some(iced::Background::Color(color::ACCENT)),
                    ..Default::default()
                }),
        )
    } else {
        None
    };

    let row_with_indicator = if let Some(ind) = indicator {
        row![ind, row_content.spacing(spacing::SM)].width(Length::Fill)
    } else {
        row![row_content].width(Length::Fill)
    };

    let base = container(row_with_indicator)
        .padding([spacing::SM, spacing::MD])
        .width(Length::Fill);

    if let Some(msg) = on_press {
        let btn_elem: Element<'_, Message> = base.into();
        iced::widget::button(btn_elem)
            .on_press(msg)
            .style(move |_theme, status| {
                let bg = match status {
                    iced::widget::button::Status::Hovered => {
                        Some(iced::Background::Color(color::FILL_HOVER))
                    }
                    iced::widget::button::Status::Pressed => {
                        Some(iced::Background::Color(color::FILL_ACTIVE))
                    }
                    _ if active => {
                        Some(iced::Background::Color(color::FILL_SELECTED))
                    }
                    _ => None,
                };
                iced::widget::button::Style {
                    background: bg,
                    text_color: color::TEXT_PRIMARY,
                    border: iced::Border::default(),
                    shadow: iced::Shadow::default(),
                    ..Default::default()
                }
            })
            .into()
    } else {
        base.into()
    }
}

/// Simple list section header — sticky-style label.
pub fn dq_list_header<'a, Message: 'a>(label: &'a str) -> Element<'a, Message> {
    container(
        text(label)
            .size(typography::CAPTION)
            .color(color::TEXT_TERTIARY),
    )
    .padding([spacing::XS, spacing::MD])
    .width(Length::Fill)
    .into()
}

/// Empty list state — centered placeholder.
pub fn dq_list_empty<'a, Message: 'a>(label: &'a str) -> Element<'a, Message> {
    container(
        text(label)
            .size(typography::BODY)
            .color(color::TEXT_TERTIARY),
    )
    .padding(spacing::XL)
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .into()
}
