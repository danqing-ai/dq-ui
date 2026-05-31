use dq_tokens::{spacing, typography};
use iced::widget::{container, text};
use iced::{Alignment, Element, Length};

/// Vertical padding so bordered pick_list / text_input layout height == CONTROL_HEIGHT.
pub fn control_padding_y() -> f32 {
    (spacing::CONTROL_HEIGHT - typography::BODY) / 2.0
}

pub fn control_padding() -> [f32; 2] {
    [control_padding_y(), spacing::CONTROL_PADDING_X]
}

/// Label centered in a fixed-height control button.
pub fn centered_button_label<'a, Message: 'a>(
    label: impl text::IntoFragment<'a>,
) -> Element<'a, Message> {
    container(
        text(label)
            .size(typography::LABEL)
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .align_x(Alignment::Center)
    .align_y(Alignment::Center)
    .into()
}
