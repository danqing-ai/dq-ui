use crate::styles::mode_tab_pill;
use dq_tokens::{color, typography};
use dq_theme::mode_tabs_container;
use iced::widget::{button, container, row, text};
use iced::{Alignment, Element, Length};

#[derive(Debug, Clone)]
pub struct ModeTabOption<T> {
    pub label: String,
    pub value: T,
}

/// Mode tabs — pill style with hover fill, no bottom hairline.
pub fn dq_mode_tabs<'a, T, Message>(
    options: &'a [ModeTabOption<T>],
    active: &'a T,
    on_select: impl Fn(T) -> Message + Clone + 'a,
) -> Element<'a, Message>
where
    T: PartialEq + Clone + 'a,
    Message: Clone + 'a,
{
    let tabs: Vec<Element<'a, Message>> = options
        .iter()
        .map(|opt| {
            let is_active = &opt.value == active;
            let on_select = on_select.clone();
            let value = opt.value.clone();

            button(
                text(opt.label.as_str())
                    .size(typography::LABEL)
                    .color(if is_active {
                        color::text_primary()
                    } else {
                        color::text_secondary()
                    }),
            )
            .padding([6.0, 10.0])
            .style(mode_tab_pill(is_active))
            .on_press(on_select(value))
            .into()
        })
        .collect();

    container(
        row(tabs)
            .spacing(2.0)
            .align_y(Alignment::Center)
            .width(Length::Fill),
    )
    .width(Length::Fill)
    .padding(0.0)
    .style(mode_tabs_container)
    .into()
}
