use dq_tokens::{color, typography};
use iced::widget::{container, text};
use iced::Element;

pub fn badge<'a, Message: 'a>(label: impl text::IntoFragment<'a>) -> Element<'a, Message> {
    container(text(label).size(typography::CAPTION).color(color::text_secondary()))
        .padding([2, 6])
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(color::bg_surface())),
            border: iced::Border {
                color: color::border_subtle(),
                width: 1.0,
                radius: 999.0.into(),
            },
            ..Default::default()
        })
        .into()
}
