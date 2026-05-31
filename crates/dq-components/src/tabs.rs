use crate::styles::tab_button;
use iced::widget::{button, row, text};
use iced::{Element, Length};

#[derive(Debug, Clone)]
pub struct TabOption<T> {
    pub label: String,
    pub value: T,
}

pub fn dq_tabs<'a, T, Message>(
    options: &'a [TabOption<T>],
    active: &'a T,
    on_select: impl Fn(T) -> Message + Clone + 'a,
) -> Element<'a, Message>
where
    T: PartialEq + Clone + 'a,
    Message: Clone + 'a,
{
    let tabs = options.iter().map(|opt| {
        let is_active = &opt.value == active;
        let on_select = on_select.clone();
        let value = opt.value.clone();
        button(text(opt.label.as_str()).size(dq_tokens::typography::LABEL))
            .width(Length::Fill)
            .padding([5.0, 10.0])
            .style(tab_button(is_active))
            .on_press(on_select(value))
            .into()
    });

    container_row(tabs.collect())
}

fn container_row<'a, Message: 'a>(children: Vec<Element<'a, Message>>) -> Element<'a, Message> {
    use iced::widget::container;
    use dq_tokens::color;

    container(row(children).spacing(2).width(Length::Fill))
        .padding(2)
        .width(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(color::BG_ELEVATED)),
            border: iced::Border {
                color: color::BORDER_SUBTLE,
                width: 1.0,
                radius: dq_tokens::spacing::RADIUS_MD.into(),
            },
            ..Default::default()
        })
        .into()
}
