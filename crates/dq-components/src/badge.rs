use dq_tokens::{color, typography};
use iced::widget::{container, text};
use iced::Element;

pub fn badge<'a, Message: 'a>(label: impl text::IntoFragment<'a>) -> Element<'a, Message> {
    container(text(label).size(typography::CAPTION).color(color::TEXT_SECONDARY))
        .padding([2, 6])
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(color::BG_SURFACE)),
            border: iced::Border {
                color: color::BORDER_SUBTLE,
                width: 1.0,
                radius: 999.0.into(),
            },
            ..Default::default()
        })
        .into()
}
