use dq_tokens::color;
use iced::widget::{container, row, Space};
use iced::{Alignment, Element, Length};

/// Split view — resizable-ish columns (static widths for now).
/// Usage: sidebar | divider | main | divider | detail.
pub fn dq_split_view<'a, Message: 'a>(
    left: Element<'a, Message>,
    center: Element<'a, Message>,
    right: Element<'a, Message>,
    left_width: Length,
    center_width: Length,
    right_width: Length,
) -> Element<'a, Message> {
    row![
        container(left).width(left_width).height(Length::Fill),
        divider(),
        container(center).width(center_width).height(Length::Fill),
        divider(),
        container(right).width(right_width).height(Length::Fill),
    ]
    .spacing(0.0)
    .align_y(Alignment::Start)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

/// Two-column split — sidebar + main.
pub fn dq_split_view_two<'a, Message: 'a>(
    left: Element<'a, Message>,
    right: Element<'a, Message>,
    left_width: Length,
) -> Element<'a, Message> {
    row![
        container(left).width(left_width).height(Length::Fill),
        divider(),
        container(right).width(Length::Fill).height(Length::Fill),
    ]
    .spacing(0.0)
    .align_y(Alignment::Start)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn divider<'a, Message: 'a>() -> iced::Element<'a, Message> {
    container(Space::new())
        .width(Length::Fixed(1.0))
        .height(Length::Fill)
        .style(|_theme| iced::widget::container::Style {
            background: Some(iced::Background::Color(color::SEPARATOR)),
            ..Default::default()
        })
        .into()
}
