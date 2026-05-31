use crate::control::control_padding;
use crate::styles::pick_list_style;
use iced::widget::pick_list;
use iced::{Element, Length};

pub fn dq_pick_list<'a, T, Message>(
    options: &'a [T],
    selected: Option<&'a T>,
    on_select: impl Fn(T) -> Message + 'a,
    placeholder: &str,
) -> Element<'a, Message>
where
    T: ToString + PartialEq + Clone + 'a,
    Message: Clone + 'a,
{
    pick_list(options, selected.cloned(), on_select)
        .placeholder(placeholder)
        .padding(control_padding())
        .text_size(dq_tokens::typography::BODY)
        .text_line_height(iced::widget::text::LineHeight::Relative(1.0))
        .width(Length::Fill)
        .style(pick_list_style)
        .into()
}
