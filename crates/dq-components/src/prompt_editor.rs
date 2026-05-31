use crate::input::dq_text_editor_with_counter;
use dq_tokens::spacing;
use iced::widget::{column, container, row, text, text_editor};
use iced::{Alignment, Element, Length};

/// Preset picker + load action on one control row (preset is pick_list placeholder, not a label).
pub fn dq_prompt_preset_row<'a, Message: 'a>(
    preset_picker: Element<'a, Message>,
    load_action: Element<'a, Message>,
) -> Element<'a, Message> {
    row![
        container(preset_picker)
            .width(Length::Fill)
            .height(Length::Fill),
        load_action,
    ]
    .spacing(spacing::CONTROL_ROW_GAP)
    .align_y(Alignment::Center)
    .height(Length::Fixed(spacing::CONTROL_HEIGHT))
    .width(Length::Fill)
    .into()
}

/// Prompt panel body: preset row + multiline editor with inset counter.
pub fn dq_prompt_editor<'a, Message: Clone + 'a>(
    preset_row: Element<'a, Message>,
    placeholder: &'a str,
    content: &'a text_editor::Content,
    on_action: impl Fn(text_editor::Action) -> Message + 'a,
    count_label: impl text::IntoFragment<'a>,
) -> Element<'a, Message> {
    column![
        preset_row,
        dq_text_editor_with_counter(
            placeholder,
            content,
            on_action,
            spacing::EDITOR_HEIGHT_PROMPT,
            count_label,
        ),
    ]
    .spacing(spacing::CONTROL_BLOCK_GAP)
    .width(Length::Fill)
    .align_x(Alignment::Start)
    .into()
}
