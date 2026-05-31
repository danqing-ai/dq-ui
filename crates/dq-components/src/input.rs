use crate::control::control_padding;
use crate::styles::{text_editor_style, text_input_style};
use dq_tokens::{color, spacing, typography};
use iced::widget::{stack, text, text_editor, text_input};
use iced::{Alignment, Element, Length};

pub fn dq_text_input<'a, Message: Clone + 'a>(
    placeholder: &str,
    value: &str,
    on_input: impl Fn(String) -> Message + 'a,
) -> Element<'a, Message> {
    text_input(placeholder, value)
        .on_input(on_input)
        .padding(control_padding())
        .size(typography::BODY)
        .line_height(iced::widget::text::LineHeight::Relative(1.0))
        .width(Length::Fill)
        .style(text_input_style)
        .into()
}

fn dq_text_editor_inner<'a, Message: Clone + 'a>(
    placeholder: &'a str,
    content: &'a text_editor::Content,
    on_action: impl Fn(text_editor::Action) -> Message + 'a,
    height: f32,
) -> Element<'a, Message> {
    text_editor(content)
        .placeholder(placeholder)
        .on_action(on_action)
        .padding([8.0, spacing::CONTROL_PADDING_X])
        .size(typography::BODY)
        .line_height(iced::widget::text::LineHeight::Relative(1.4))
        .height(Length::Fixed(height))
        .style(text_editor_style)
        .into()
}

pub fn dq_text_editor<'a, Message: Clone + 'a>(
    placeholder: &'a str,
    content: &'a text_editor::Content,
    on_action: impl Fn(text_editor::Action) -> Message + 'a,
    height: f32,
) -> Element<'a, Message> {
    dq_text_editor_inner(placeholder, content, on_action, height)
}

pub fn dq_text_editor_with_counter<'a, Message: Clone + 'a>(
    placeholder: &'a str,
    content: &'a text_editor::Content,
    on_action: impl Fn(text_editor::Action) -> Message + 'a,
    height: f32,
    count_label: impl text::IntoFragment<'a>,
) -> Element<'a, Message> {
    let editor = dq_text_editor_inner(placeholder, content, on_action, height);

    let counter = iced::widget::container(
        text(count_label)
            .size(typography::MINI)
            .color(color::TEXT_QUATERNARY),
    )
    .width(Length::Fill)
    .height(Length::Fixed(height))
    .align_x(Alignment::End)
    .align_y(Alignment::End)
    .padding([5.0, spacing::CONTROL_PADDING_X]);

    stack![editor, counter]
        .width(Length::Fill)
        .height(Length::Fixed(height))
        .into()
}

pub fn dq_text_input_multiline<'a, Message: Clone + 'a>(
    placeholder: &'a str,
    content: &'a text_editor::Content,
    on_action: impl Fn(text_editor::Action) -> Message + 'a,
    count_label: impl text::IntoFragment<'a>,
) -> Element<'a, Message> {
    dq_text_editor_with_counter(
        placeholder,
        content,
        on_action,
        spacing::EDITOR_HEIGHT_NEGATIVE,
        count_label,
    )
}
