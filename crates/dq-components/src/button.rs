use crate::control::centered_button_label;
use crate::styles::{control_inset_button, ghost_button, primary_button, secondary_button};
use dq_tokens::{spacing, typography};
use iced::widget::{button, container, row, text};
use iced::{Alignment, Element, Length};

#[derive(Debug, Clone, Copy, Default)]
pub enum ButtonVariant {
    #[default]
    Secondary,
    Primary,
    Ghost,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ButtonSize {
    Sm,
    #[default]
    Md,
    Lg,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ButtonWidth {
    #[default]
    Hug,
    Fill,
}

impl ButtonSize {
    fn height(self) -> f32 {
        match self {
            ButtonSize::Sm => spacing::BUTTON_HEIGHT_SM,
            ButtonSize::Md => spacing::CONTROL_HEIGHT,
            ButtonSize::Lg => spacing::BUTTON_HEIGHT_LG,
        }
    }
}

fn fixed_height_button<'a, Message: Clone + 'a>(
    content: Element<'a, Message>,
    height: f32,
    width: Length,
    style: fn(&iced::Theme, button::Status) -> button::Style,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    button(content)
        .padding([0.0, 0.0])
        .width(width)
        .height(Length::Fixed(height))
        .style(style)
        .on_press_maybe(on_press)
        .into()
}

pub fn dq_button<'a, Message: Clone + 'a>(
    label: impl text::IntoFragment<'a>,
    variant: ButtonVariant,
    size: ButtonSize,
    width: ButtonWidth,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let style = match variant {
        ButtonVariant::Primary => primary_button,
        ButtonVariant::Secondary => secondary_button,
        ButtonVariant::Ghost => ghost_button,
    };

    let button_width = match width {
        ButtonWidth::Fill => Length::Fill,
        ButtonWidth::Hug => Length::Shrink,
    };

    fixed_height_button(
        centered_button_label(label),
        size.height(),
        button_width,
        style,
        on_press,
    )
}

/// Section header text action — 28px, ghost.
pub fn dq_header_button<'a, Message: Clone + 'a>(
    label: impl text::IntoFragment<'a>,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    fixed_height_button(
        centered_button_label(label),
        spacing::BUTTON_HEIGHT_SM,
        Length::Shrink,
        ghost_button,
        on_press,
    )
}

/// 32px inset button for control rows (e.g. 加载 beside pick_list).
pub fn dq_control_button<'a, Message: Clone + 'a>(
    label: impl text::IntoFragment<'a>,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    fixed_height_button(
        centered_button_label(label),
        spacing::CONTROL_HEIGHT,
        Length::Fixed(spacing::LOAD_BUTTON_WIDTH),
        control_inset_button,
        on_press,
    )
}

/// Full-width primary CTA — label centered in 36px bar.
pub fn dq_primary_action<'a, Message: Clone + 'a>(
    label: impl text::IntoFragment<'a>,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    use crate::sparkle_icon;
    use iced::Color;

    let content = container(
        row![
            sparkle_icon::<Message>(Color::WHITE),
            text(label).size(typography::LABEL),
        ]
        .spacing(6.0)
        .align_y(Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .align_x(Alignment::Center)
    .align_y(Alignment::Center);

    fixed_height_button(
        content.into(),
        spacing::BUTTON_HEIGHT_LG,
        Length::Fill,
        primary_button,
        on_press,
    )
}
