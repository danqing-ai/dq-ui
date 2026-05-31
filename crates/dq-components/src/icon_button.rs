use crate::icons::{section_icon, SectionIcon};
use crate::phosphor::{phosphor_icon, PhosphorIcon};
use dq_tokens::{color, spacing};
use iced::widget::{button, container};
use iced::{Alignment, Element, Length};

pub fn icon_button<'a, Message: Clone + 'a>(
    icon: SectionIcon,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    button(section_icon(icon))
        .padding([0.0, 0.0])
        .width(iced::Length::Fixed(spacing::ICON_BUTTON_SIZE))
        .height(iced::Length::Fixed(spacing::ICON_BUTTON_SIZE))
        .style(icon_button_style)
        .on_press_maybe(on_press)
        .into()
}

/// Generic icon button using Phosphor icons — more flexible than SectionIcon-based.
pub fn phosphor_icon_button<'a, Message: Clone + 'a>(
    icon: PhosphorIcon,
    icon_size: f32,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    button(
        container(phosphor_icon(icon, icon_size, color::TEXT_SECONDARY))
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center),
    )
    .padding([0.0, 0.0])
    .width(iced::Length::Fixed(spacing::ICON_BUTTON_SIZE))
    .height(iced::Length::Fixed(spacing::ICON_BUTTON_SIZE))
    .style(icon_button_style)
    .on_press_maybe(on_press)
    .into()
}

fn icon_button_style(_theme: &iced::Theme, status: iced::widget::button::Status) -> iced::widget::button::Style {
    let base = button::Style {
        background: Some(iced::Background::Color(color::BG_INSET)),
        text_color: color::TEXT_SECONDARY,
        border: iced::Border {
            color: color::BORDER_SUBTLE,
            width: 1.0,
            radius: spacing::RADIUS_MD.into(),
        },
        ..Default::default()
    };
    match status {
        button::Status::Hovered => button::Style {
            background: Some(iced::Background::Color(color::BG_ELEVATED)),
            border: iced::Border {
                color: color::BORDER_STRONG,
                ..base.border
            },
            ..base
        },
        _ => base,
    }
}
